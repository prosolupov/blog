use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogError {
    #[error("Пользователь или блог не найдены")]
    NotFound,
    #[error("Пользователем с таким email {0} уже существует")]
    AlreadyExists(String),
    #[error("Ошибка авторизации")]
    InvalidCredential,
    #[error("Ошибка валидации данных: {0}")]
    Validation(String),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error("Internal server error")]
    Internal,
}

impl ResponseError for BlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            BlogError::Validation(_) => StatusCode::BAD_REQUEST,
            BlogError::NotFound => StatusCode::NOT_FOUND,
            BlogError::AlreadyExists(_) => StatusCode::CONFLICT,
            BlogError::InvalidCredential => StatusCode::BAD_REQUEST,
            Self::Internal => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
