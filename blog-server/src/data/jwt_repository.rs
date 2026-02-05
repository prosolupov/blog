use crate::domain::error::BlogError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{query_as, query_scalar, PgPool};
use uuid::Uuid;
use crate::data::post_repository::PostgresPostRepo;

#[async_trait]
pub trait JwtRepository: Send + Sync {
    async fn create_token(&self, id: Uuid, user_id: Uuid, expires_at: DateTime<Utc>) -> Result<Uuid, BlogError>;
    async fn find_by_id(&self, id: Uuid) -> Result<String, BlogError>;
}


#[derive(Clone)]
pub struct PostgresUserRepo {
    pub pool: PgPool,
}

#[async_trait]
impl JwtRepository for PostgresPostRepo {
    async fn create_token(&self, id: Uuid, user_id: Uuid, expires_at: DateTime<Utc>) -> Result<Uuid, BlogError> {
        let new_token = query_scalar!(
            "INSERT INTO refresh_tokens (id, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id",
            id,
            user_id,
            expires_at
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(new_token)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<String, BlogError> {
        todo!()
    }
}
