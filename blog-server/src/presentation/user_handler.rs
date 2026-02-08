use crate::application::user_service::UserService;
use crate::domain::error::BlogError;
use crate::domain::user::UserRegistration;
use actix_web::{HttpResponse, Responder, post, web};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/user").service(create_user));
}

#[post("/register")]
async fn create_user(
    user_service: web::Data<UserService>,
    user: web::Json<UserRegistration>,
) -> impl Responder {
    match user_service.create_new_user(user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => match err {
            BlogError::AlreadyExists(_) => HttpResponse::Conflict().json(format!("Error")),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
