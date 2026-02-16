use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::ReqData;
use uuid::Uuid;
use crate::application::post_service::PostService;
use crate::domain::error::BlogError;
use crate::domain::jwt::AccessClaims;
use crate::domain::post::{CreatePost, Pagination, PostId};
use crate::domain::user::UserId;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_post)
        .service(get_post)
        .service(get_list_posts)
        .service(update_post)
        .service(delete_post);
}

#[post("/posts")]
async fn create_post(
    post_service: web::Data<PostService>,
    post: web::Json<CreatePost>,
    access_claims: ReqData<AccessClaims>
) -> impl Responder {
    match post_service.create_post(UserId(access_claims.sub), post.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => match e {
            BlogError::NotFound => HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" })),
            _ => {HttpResponse::InternalServerError().finish()}
        }
    }
}

#[get("/posts/{id}")]
async fn get_post(
    post_service: web::Data<PostService>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match post_service.get_post(PostId(id.into_inner())).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => {HttpResponse::NotFound().json(serde_json::json!({ "error": err.to_string() }))}
    }
}

#[get("/posts")]
async fn get_list_posts(
    post_service: web::Data<PostService>,
    query: web::Query<Pagination>,
) -> impl Responder {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 20);

    match post_service.get_list_posts(page, per_page).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {HttpResponse::NotFound().json(serde_json::json!({ "error": err.to_string() }))}
    }
}

#[put("/posts/{id}")]
async fn update_post(
    post_service: web::Data<PostService>,
    access_claims: ReqData<AccessClaims>,
    id: web::Path<Uuid>,
    payload: web::Json<CreatePost>,
) -> impl Responder {
    match post_service.update_post_by_id(UserId(access_claims.sub), PostId(id.into_inner()), payload.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => {
            match err {
                BlogError::NotFound => HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" })),
                _ => {HttpResponse::InternalServerError().finish()}
            }
        }
    }
}

#[delete("/posts/{id}")]
async fn delete_post(
    post_service: web::Data<PostService>,
    id: web::Path<Uuid>,
    access_claims: ReqData<AccessClaims>,
) -> impl Responder {
    match post_service.delete_post_by_id(UserId(access_claims.sub), PostId(id.into_inner())).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => {
            match err {
                BlogError::NotFound => HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" })),
                _ => {HttpResponse::InternalServerError().finish()}
            }
        }
    }
}

