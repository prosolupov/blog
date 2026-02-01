use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Serialize, Debug, Validate)]
pub struct User {
    id: UserId,
    username: String,
    #[validate(email(message = "Некорректный формат email"))]
    email: String,
    #[validate(length(min = 8, message = "Пароль должен быть не менее 8 символов"))]
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserAuthorization {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
}

#[derive(Serialize)]
pub struct UserResponseAuthentication {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdate {
    username: String,
    #[validate(email(message = "Некорректный формат email"))]
    email: String,
    #[validate(length(min = 8, message = "Пароль должен быть не менее 8 символов"))]
    password: String,
}
