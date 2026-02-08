pub mod jwt_repository;
pub mod post_repository;
pub mod user_repository;

use crate::data::jwt_repository::JwtRepository;
pub use post_repository::PostRepository;
use std::sync::Arc;
pub use user_repository::UserRepository;

pub struct RepositoryContainer {
    pub user: Arc<dyn UserRepository>,
    pub post: Arc<dyn PostRepository>,
    pub jwt: Arc<dyn JwtRepository>,
}
