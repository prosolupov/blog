use crate::domain::error::{AuthError, BlogError};
use crate::domain::jwt::{AccessClaims, RefreshClaims};
use chrono::{Duration, Utc};
use jsonwebtoken::Validation;
use serde::de::DeserializeOwned;
use std::env;
use uuid::Uuid;

fn create_exp(period: Duration) -> usize {
    let refresh_exp = Utc::now()
        .checked_add_signed(period)
        .expect("valid timestamp")
        .timestamp() as usize;

    refresh_exp
}

fn get_token_key() -> String {
    let key = env::var("TOKEN_KEY").expect("TOKEN_KEY must be set");
    key
}

pub fn create_refresh_token(user_id: Uuid, iat: usize) -> Result<(String, Uuid, usize), BlogError> {
    let key = get_token_key();
    let jti = Uuid::new_v4();
    let exp = create_exp(Duration::days(7));

    let refresh_token = RefreshClaims {
        sub: user_id,
        iat,
        exp: exp.clone(),
        jti: jti.clone(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &refresh_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    )
    .map_err(|_| BlogError::Internal)?;

    Ok((token, jti, exp))
}

pub fn create_access_token(
    user_id: Uuid,
    user_name: String,
    iat: usize,
) -> Result<String, BlogError> {
    let key = get_token_key();

    let access_token = AccessClaims {
        sub: user_id,
        iat,
        username: user_name,
        exp: create_exp(Duration::minutes(15)),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &access_token,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    )
    .map_err(|_| BlogError::Internal)?;

    Ok(token)
}

pub fn decode_token<T: DeserializeOwned>(token: String) -> Result<T, BlogError> {
    let key = get_token_key();

    let token = jsonwebtoken::decode::<T>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(key.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(AuthError::from)
    .map_err(BlogError::from)?;

    Ok(token.claims)
}
