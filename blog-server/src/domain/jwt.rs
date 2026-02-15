use crate::domain::user::{UserResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}