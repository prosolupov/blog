use crate::domain::jwt::{AccessClaims, RefreshClaims};
use actix_web::web::to;
use dotenvy::dotenv;
use jsonwebtoken::errors::Error;
use std::env;
use chrono::{Duration, Utc};
use uuid::Uuid;

fn create_exp(period: Duration) -> usize {
    let refresh_exp = Utc::now()
        .checked_add_signed(period)
        .expect("valid timestamp")
        .timestamp() as usize;

    refresh_exp

}

async fn create_refresh_token(user_id: Uuid) -> Result<String, Error> {
    let key = env::var("REFRESH_KEY").expect("REFRESH_KEY must be set");

    let expiration_time = Utc::now() + Duration::days(7);

    let refresh_token = RefreshClaims {
        sub: user_id,
        exp: create_exp(Duration::days(7)),
        jti: Uuid::new_v4().to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    );

    token
}

async fn create_access_token(user_id: Uuid, user_name: String) -> Result<String, Error> {
    let access_token = AccessClaims {
        sub: user_id,
        username: user_name,
        exp: create_exp(Duration::minutes(15)),
    };

    let key = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    );

    token
}
