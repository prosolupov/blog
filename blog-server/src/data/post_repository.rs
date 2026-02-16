use async_trait::async_trait;
use sqlx::{query_as, PgPool};
use crate::domain::error::BlogError;
use crate::domain::post::{CreatePost, PostId, ResponsePost};
use crate::domain::user::UserId;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, author_id: UserId, post: CreatePost) -> Result<ResponsePost, BlogError>;
    async fn get_post_by_id(&self, id: PostId) -> Result<ResponsePost, BlogError>;
    async fn get_list_posts(&self, page: u32, per_page: u32) -> Result<Vec<ResponsePost>, BlogError>;
    async fn update_post_by_id(&self, author_id: UserId, post_id: PostId, payload: CreatePost) -> Result<ResponsePost, BlogError>;
    async fn delete_post_by_id(&self, author_id: UserId, post_id: PostId) -> Result<(), BlogError>;
}

#[derive(Clone)]
pub struct PostgresPostRepo {
    pub pool: PgPool,
}
#[async_trait]
impl PostRepository for PostgresPostRepo {
    async fn create_post(&self, author_id: UserId, post: CreatePost) -> Result<ResponsePost, BlogError> {
        let new_post = query_as!(
            ResponsePost,
            "INSERT INTO posts (title, content, author_id) VALUES ($1, $2, $3) RETURNING id, title, content, author_id",
            post.title,
            post.content,
            author_id.as_uuid(),
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(new_post)
    }

    async fn get_post_by_id(&self, id: PostId) -> Result<ResponsePost, BlogError> {
        let post = query_as!(
            ResponsePost,
            "SELECT id, title, content, author_id FROM posts WHERE id = $1",
            id.as_uuid()
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(post)
    }

    async fn get_list_posts(&self, page: u32, per_page: u32) -> Result<Vec<ResponsePost>, BlogError> {
        let offset = ((page - 1) * per_page) as i64;
        let limit = per_page as i64;

        let posts = query_as!(
            ResponsePost,
            "SELECT id, title, content, author_id FROM posts ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(posts)
    }

    async fn update_post_by_id(&self, author_id: UserId, post_id: PostId, payload: CreatePost) -> Result<ResponsePost, BlogError> {
        let post = query_as!(
            ResponsePost,
            "UPDATE posts SET title = $1, content = $2 WHERE id = $3 AND author_id = $4 RETURNING id, title, content, author_id",
            payload.title,
            payload.content,
            post_id.as_uuid(),
            author_id.as_uuid(),

        ).fetch_one(&self.pool).await?;

        Ok(post)
    }

    async fn delete_post_by_id(&self, author_id: UserId, post_id: PostId) -> Result<(), BlogError> {
        let _post = query_as!(
            ResponsePost,
            "DELETE FROM posts WHERE id = $1 AND author_id = $2 RETURNING id, title, content, author_id",
            post_id.as_uuid(),
            author_id.as_uuid(),
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(())
    }
}
