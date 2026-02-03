use crate::domain::jwt::{AccessClaims, RefreshClaims};
use actix_web::web::to;
use dotenvy::dotenv;
use jsonwebtoken::errors::Error;
use std::env;
use chrono::{Duration, Utc};
use uuid::Uuid;

fn create_exp() -> String {
    todo!()
}

fn create_refresh_token(user_id: Uuid) -> Result<String, Error> {
    let key = env::var("REFRESH_KEY").expect("REFRESH_KEY must be set");

    let refresh_exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let refresh_token = RefreshClaims {
        sub: user_id,
        exp: refresh_exp,
        jti: Uuid::new_v4().to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    );

    token
}

fn create_access_token(user_id: Uuid, user_name: String) -> Result<String, Error> {
    let expiration_time = Utc::now() + Duration::days(7);

    let access_exp = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp() as usize;

    let access_token = AccessClaims {
        sub: user_id,
        username: user_name,
        exp: access_exp,
    };

    let key = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    );

    token
}
