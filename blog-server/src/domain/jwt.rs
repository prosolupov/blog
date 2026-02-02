use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub jti: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub username: String,
    pub exp: i64,
}