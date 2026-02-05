use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::data::jwt_repository::JwtRepository;
use crate::domain::error::BlogError;

pub struct JwtService {
    repo: Box<dyn JwtRepository>,
}

impl JwtService {
    pub fn new(repo: Box<dyn JwtRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn create_refresh_token(&self, id: Uuid, user_id: Uuid, expires_at: DateTime<Utc>) -> Result<Uuid, BlogError> {
        let token = self.repo.create_token(id, user_id, expires_at).await;
        token
    }
}