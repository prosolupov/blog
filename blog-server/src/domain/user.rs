use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

#[derive(Serialize)]
struct Users {
    id: UserId,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct UserRegistration {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserLogin {
    username: String,
    password: String
}