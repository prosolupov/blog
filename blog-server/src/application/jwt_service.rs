use crate::data::UserRepository;
use crate::data::jwt_repository::JwtRepository;
use crate::domain::error::{AuthError, BlogError};
use crate::domain::jwt::{RefreshClaims};
use crate::domain::user::UserResponse;
use crate::infrastructure::jwt::{create_access_token, create_refresh_token, decode_token};
use argon2::Argon2;
use chrono::{DateTime, Utc};
use jsonwebtoken::signature::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};
use std::sync::Arc;
use uuid::Uuid;

pub struct JwtService {
    jwt_repo: Arc<dyn JwtRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl JwtService {
    pub fn new(jwt_repo: Arc<dyn JwtRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self {
            jwt_repo,
            user_repo,
        }
    }

    fn create_hash_token(token: String) -> Result<String, BlogError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();

        let hash = argon
            .hash_password(token.as_bytes(), &salt)
            .map_err(|_| BlogError::Internal)?;

        Ok(hash.to_string())
    }

    fn usize_to_datetime(exp: usize) -> Result<DateTime<Utc>, AuthError> {
        let date_time = DateTime::from_timestamp(exp as i64, 0).ok_or(AuthError::InvalidToken)?;

        Ok(date_time)
    }

    pub async fn create_token(
        &self,
        user_id: Uuid,
        user_name: String,
    ) -> Result<(String, String), BlogError> {
        let iat = Utc::now().timestamp() as usize;
        let (refresh_token, jti, exp) = create_refresh_token(user_id, iat)?;
        let refresh_token_hash = Self::create_hash_token(refresh_token.clone())?;
        let expires_at = Self::usize_to_datetime(exp)?;
        self.jwt_repo
            .save_refresh_token(jti, user_id, refresh_token_hash, expires_at)
            .await?;

        let accses_token = create_access_token(user_id, user_name, iat)?;

        Ok((accses_token, refresh_token))
    }

    pub async fn refresh(
        &self,
        token: String,
    ) -> Result<(String, String, UserResponse), BlogError> {
        let refresh_claims = decode_token::<RefreshClaims>(token)?;
        self.jwt_repo.find_refresh_token(refresh_claims.jti).await?;
        let user = self.user_repo.get_user_by_id(refresh_claims.sub).await?;
        self.jwt_repo
            .delete_refresh_token(refresh_claims.jti)
            .await?;
        let (access_token, refresh_token) = self
            .create_token(refresh_claims.sub, user.username.clone())
            .await?;
        Ok((access_token, refresh_token, user))
    }

    pub async fn delete_token(&self, refresh_token: String) -> Result<(), BlogError> {
        let refresh_token = decode_token::<RefreshClaims>(refresh_token)?;
        self.jwt_repo.find_refresh_token(refresh_token.sub).await?;
        Ok(())
    }
}
