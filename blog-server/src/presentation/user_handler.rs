use crate::application::user_service::UserService;
use crate::domain::error::BlogError;
use crate::domain::user::{UserAuthorization, UserRegistration};
use actix_web::{HttpResponse, Responder, get, post, web};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth").service(create_user).service(login));
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

#[post("/login")]
async fn login(
    user_service: web::Data<UserService>,
    user: web::Json<UserAuthorization>,
) -> impl Responder {
    match user_service.login(user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => match err {
            BlogError::InvalidCredential => HttpResponse::BadRequest().json(format!("Error")),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
