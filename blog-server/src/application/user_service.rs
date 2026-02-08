use crate::data::UserRepository;
use crate::domain::error::BlogError;
use crate::domain::user::{UserRegistration, UserResponse};
use crate::infrastructure::password::password_hash;
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_new_user(
        &self,
        mut user: UserRegistration,
    ) -> Result<UserResponse, BlogError> {
        user.password = password_hash(&user.password)?;
        Ok(self.repo.create_user(user).await?)
    }
}
