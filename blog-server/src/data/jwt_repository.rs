use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::BlogError;


#[async_trait]
pub trait JwtRepository: Send + Sync {
    async fn create_token(&self, user_id: Uuid, expires_in: i64) -> Result<Uuid, BlogError>;
    async fn find_by_id(&self, id: Uuid) -> Result<String, BlogError>;
}