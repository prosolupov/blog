use crate::data::PostRepository;
use std::sync::Arc;
use crate::domain::error::BlogError;
use crate::domain::post::{CreatePost, PostId, ResponsePost};
use crate::domain::user::UserId;

pub struct PostService {
    repo: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_post(&self, author_id: UserId, post: CreatePost) -> Result<ResponsePost, BlogError> {
        Ok(self.repo.create_post(author_id, post).await?)
    }

    pub async fn get_post(&self, id: PostId) -> Result<ResponsePost, BlogError> {
        Ok(self.repo.get_post_by_id(id).await?)
    }

    pub async fn get_list_posts(&self, page: u32, per_page: u32) -> Result<Vec<ResponsePost>, BlogError> {
        Ok(self.repo.get_list_posts(page, per_page).await?)
    }

    pub async fn update_post_by_id(&self, author_id: UserId, post_id:PostId, payload: CreatePost) -> Result<ResponsePost, BlogError> {
        Ok(self.repo.update_post_by_id(author_id, post_id, payload).await?)
    }

    pub async fn delete_post_by_id(&self, author_id: UserId, post_id: PostId) -> Result<(), BlogError> {
        self.repo.delete_post_by_id(author_id, post_id).await?;
        Ok(())
    }

}
