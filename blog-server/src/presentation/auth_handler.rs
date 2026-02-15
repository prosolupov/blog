use crate::application::auth_service::AuthService;
use crate::domain::error::BlogError;
use crate::domain::user::UserAuthorization;
use actix_web::{HttpResponse, Responder, post, web};
use tracing_actix_web::root_span_macro::private::tracing::log::info;
use crate::domain::jwt::RefreshRequest;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth")
        .service(login)
        .service(refresh)
    );
}

#[post("/login")]
async fn login(
    auth_service: web::Data<AuthService>,
    user: web::Json<UserAuthorization>,
) -> impl Responder {
    match auth_service.login(user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => match err {
            BlogError::InvalidCredential => HttpResponse::BadRequest().json(format!("Error")),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}


#[post("/refresh")]
async fn refresh(auth_service: web::Data<AuthService>, refresh_token: web::Json<RefreshRequest>) -> impl Responder {
    info!("Refresh token: {:?}", refresh_token.refresh_token);
    match auth_service.refresh(refresh_token.refresh_token.clone()).await {
        Ok(user) => {
            info!("Successfully refreshed: {}", user.user.username);
            HttpResponse::Ok().json(user)
        },
        Err(err) => match err {
            BlogError::NotFound => HttpResponse::BadRequest().json(format!("Error")),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
