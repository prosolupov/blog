use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::application::auth_service::AuthService;
use crate::application::post_service::PostService;
use crate::application::user_service::UserService;
use crate::domain::error::BlogError;
use crate::domain::jwt::AccessClaims;
use crate::domain::post::{CreatePost, PostId, ResponsePost};
use crate::domain::user::{UserAuthorization, UserId, UserRegistration};
use crate::infrastructure::jwt::decode_token;
use crate::pb::blog_service_server::BlogService;
use crate::pb::{
    AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
    ListPostsRequest, ListPostsResponse, LoginRequest, Post, PostResponse, RegisterRequest,
    UpdatePostRequest, User,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct BlogGrpcService {
    auth: Arc<AuthService>,
    user: Arc<UserService>,
    post: Arc<PostService>,
}

impl BlogGrpcService {
    pub fn new(auth: Arc<AuthService>, user: Arc<UserService>, post: Arc<PostService>) -> Self {
        Self { auth, user, post }
    }
}

fn map_error(err: BlogError) -> Status {
    match err {
        BlogError::NotFound => Status::not_found("resource not found"),
        BlogError::InvalidCredential => Status::unauthenticated("invalid credentials"),
        BlogError::AlreadyExists(msg) => Status::already_exists(msg),
        BlogError::Validation(msg) => Status::invalid_argument(msg),
        BlogError::Auth(_) => Status::unauthenticated("invalid or expired token"),
        _ => Status::internal("internal server error"),
    }
}

fn map_auth_response(resp: crate::domain::jwt::AuthResponse) -> AuthResponse {
    AuthResponse {
        access_token: resp.access_token,
        refresh_token: resp.refresh_token,
        user: Some(User {
            id: resp.user.id.0.to_string(),
            username: resp.user.username,
            email: String::new(),
        }),
    }
}

fn map_post(post: ResponsePost) -> Post {
    Post {
        id: String::new(),
        title: post.title,
        content: post.content,
        author_id: String::new(),
        created_at: None,
        updated_at: None,
    }
}

fn parse_post_id(id: &str) -> Result<PostId, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::invalid_argument("invalid post id"))?;
    Ok(PostId(uuid))
}

fn auth_user_id<T>(request: &Request<T>) -> Result<UserId, Status> {
    let auth = request
        .metadata()
        .get("authorization")
        .ok_or_else(|| Status::unauthenticated("missing authorization metadata"))?
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid authorization metadata"))?;

    let token = auth
        .strip_prefix("Bearer ")
        .ok_or_else(|| Status::unauthenticated("expected Bearer token"))?;

    let claims: AccessClaims = decode_token(token.to_string()).map_err(map_error)?;
    Ok(UserId(claims.sub))
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let email = req.email;
        let password = req.password;

        self.user
            .create_new_user(UserRegistration {
                username: username.clone(),
                email,
                password: password.clone(),
            })
            .await
            .map_err(map_error)?;

        let auth = self
            .auth
            .login(UserAuthorization { username, password })
            .await
            .map_err(map_error)?;

        Ok(Response::new(map_auth_response(auth)))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let auth = self
            .auth
            .login(UserAuthorization {
                username: req.username,
                password: req.password,
            })
            .await
            .map_err(map_error)?;

        Ok(Response::new(map_auth_response(auth)))
    }

    async fn create_post(&self, request: Request<CreatePostRequest>) -> Result<Response<PostResponse>, Status> {
        let author_id = auth_user_id(&request)?;
        let req = request.into_inner();

        let post = self
            .post
            .create_post(
                author_id,
                CreatePost {
                    title: req.title,
                    content: req.content,
                },
            )
            .await
            .map_err(map_error)?;

        Ok(Response::new(PostResponse {
            post: Some(map_post(post)),
        }))
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<PostResponse>, Status> {
        let _ = auth_user_id(&request)?;
        let req = request.into_inner();
        let post_id = parse_post_id(&req.id)?;
        let post = self.post.get_post(post_id).await.map_err(map_error)?;

        Ok(Response::new(PostResponse {
            post: Some(map_post(post)),
        }))
    }

    async fn update_post(&self, request: Request<UpdatePostRequest>) -> Result<Response<PostResponse>, Status> {
        let author_id = auth_user_id(&request)?;
        let req = request.into_inner();
        let post_id = parse_post_id(&req.id)?;
        let post = self
            .post
            .update_post_by_id(
                author_id,
                post_id,
                CreatePost {
                    title: req.title,
                    content: req.content,
                },
            )
            .await
            .map_err(map_error)?;

        Ok(Response::new(PostResponse {
            post: Some(map_post(post)),
        }))
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, Status> {
        let author_id = auth_user_id(&request)?;
        let req = request.into_inner();
        let post_id = parse_post_id(&req.id)?;
        self.post
            .delete_post_by_id(author_id, post_id)
            .await
            .map_err(map_error)?;

        Ok(Response::new(DeletePostResponse { success: true }))
    }

    async fn list_posts(&self, request: Request<ListPostsRequest>) -> Result<Response<ListPostsResponse>, Status> {
        let _ = auth_user_id(&request)?;
        let req = request.into_inner();
        let page = if req.page == 0 { 1 } else { req.page };
        let per_page = if req.per_page == 0 { 10 } else { req.per_page };

        let items = self
            .post
            .get_list_posts(page, per_page)
            .await
            .map_err(map_error)?
            .into_iter()
            .map(map_post)
            .collect::<Vec<_>>();

        Ok(Response::new(ListPostsResponse {
            total: items.len() as u64,
            page,
            per_page,
            items,
        }))
    }
}
