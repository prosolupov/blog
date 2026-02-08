use actix_web_lab::__reexports::tracing::info;
use crate::data::post_repository::PostgresPostRepo;
use crate::domain::error::BlogError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, query_scalar, query};
use uuid::Uuid;

#[async_trait]
pub trait JwtRepository: Send + Sync {
    async fn save_refresh_token(
        &self,
        jti: Uuid,
        user_id: Uuid,
        refresh_token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> Result<Uuid, BlogError>;
    async fn find_refresh_token(&self, jti: Uuid) -> Result<Uuid, BlogError>;
    async fn delete_refresh_token(&self, jti: Uuid) -> Result<(), BlogError>;
}

#[derive(Clone)]
pub struct PostgresUserRepo {
    pub pool: PgPool,
}

#[async_trait]
impl JwtRepository for PostgresPostRepo {
    async fn save_refresh_token(
        &self,
        jti: Uuid,
        user_id: Uuid,
        refresh_token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> Result<Uuid, BlogError> {
        let new_token = query_scalar!(
            "INSERT INTO refresh_tokens (jti, user_id, refresh_token_hash, expires_at) VALUES ($1, $2, $3, $4) RETURNING jti",
            jti,
            user_id,
            refresh_token_hash,
            expires_at
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(new_token)
    }

    async fn find_refresh_token(&self, jti: Uuid) -> Result<Uuid, BlogError> {
        let jti_token = query_scalar!("SELECT jti FROM refresh_tokens WHERE jti=$1", jti)
            .fetch_one(&self.pool)
            .await?;
        Ok(jti_token)
    }

    async fn delete_refresh_token(&self, jti: Uuid) -> Result<(), BlogError> {
        query!("DELETE FROM refresh_tokens WHERE jti = $1", jti)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
