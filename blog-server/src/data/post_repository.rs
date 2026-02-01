use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait PostRepository: Send + Sync {}

#[derive(Clone)]
pub struct PostgresPostRepo {
    pub pool: PgPool,
}

impl PostRepository for PostgresPostRepo {}
