use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;

use crate::error::BlogClientError;
use crate::{AuthResponse, Post, User};

pub mod pb {
    tonic::include_proto!("blog.v1");
}

pub struct GrpcClient {
    client: pb::blog_service_client::BlogServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(endpoint: &str) -> Result<Self, BlogClientError> {
        let channel = Endpoint::from_shared(endpoint.to_string())?.connect().await?;
        Ok(Self {
            client: pb::blog_service_client::BlogServiceClient::new(channel),
        })
    }

    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let req = pb::RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        let resp = self.client.register(Request::new(req)).await?.into_inner();
        Ok(map_auth_response(resp))
    }

    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let req = pb::LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let resp = self.client.login(Request::new(req)).await?.into_inner();
        Ok(map_auth_response(resp))
    }

    pub async fn create_post(
        &mut self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let mut req = Request::new(pb::CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        });
        set_auth_metadata(req.metadata_mut(), token)?;
        let resp = self.client.create_post(req).await?.into_inner();
        map_post_response(resp)
    }

    pub async fn get_post(&mut self, token: &str, id: &str) -> Result<Post, BlogClientError> {
        let mut req = Request::new(pb::GetPostRequest {
            id: id.to_string(),
        });
        set_auth_metadata(req.metadata_mut(), token)?;
        let resp = self.client.get_post(req).await?.into_inner();
        map_post_response(resp)
    }

    pub async fn update_post(
        &mut self,
        token: &str,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let mut req = Request::new(pb::UpdatePostRequest {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        });
        set_auth_metadata(req.metadata_mut(), token)?;
        let resp = self.client.update_post(req).await?.into_inner();
        map_post_response(resp)
    }

    pub async fn delete_post(&mut self, token: &str, id: &str) -> Result<bool, BlogClientError> {
        let mut req = Request::new(pb::DeletePostRequest {
            id: id.to_string(),
        });
        set_auth_metadata(req.metadata_mut(), token)?;
        let resp = self.client.delete_post(req).await?.into_inner();
        Ok(resp.success)
    }

    pub async fn list_posts(
        &mut self,
        token: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, BlogClientError> {
        if limit == 0 {
            return Err(BlogClientError::InvalidRequest(
                "limit must be > 0".to_string(),
            ));
        }

        let page = offset / limit + 1;
        let mut req = Request::new(pb::ListPostsRequest {
            page,
            per_page: limit,
        });
        set_auth_metadata(req.metadata_mut(), token)?;
        let resp = self.client.list_posts(req).await?.into_inner();
        Ok(resp.items.into_iter().map(map_post).collect())
    }
}

fn set_auth_metadata(
    metadata: &mut tonic::metadata::MetadataMap,
    token: &str,
) -> Result<(), BlogClientError> {
    let value: MetadataValue<_> = format!("Bearer {token}")
        .parse()
        .map_err(|e| BlogClientError::InvalidRequest(format!("invalid token header: {e}")))?;
    metadata.insert("authorization", value);
    Ok(())
}

fn map_auth_response(resp: pb::AuthResponse) -> AuthResponse {
    let user = resp.user.map(|u| User {
        id: u.id,
        username: u.username,
        email: if u.email.is_empty() { None } else { Some(u.email) },
    });

    AuthResponse {
        access_token: resp.access_token,
        refresh_token: resp.refresh_token,
        user,
    }
}

fn map_post_response(resp: pb::PostResponse) -> Result<Post, BlogClientError> {
    let post = resp
        .post
        .ok_or_else(|| BlogClientError::InvalidRequest("missing post in response".to_string()))?;
    Ok(map_post(post))
}

fn map_post(post: pb::Post) -> Post {
    Post {
        id: if post.id.is_empty() { None } else { Some(post.id) },
        title: post.title,
        content: post.content,
        author_id: if post.author_id.is_empty() {
            None
        } else {
            Some(post.author_id)
        },
    }
}
