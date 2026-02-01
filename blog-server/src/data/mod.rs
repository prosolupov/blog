pub mod post_repository;
pub mod user_repository;

pub use post_repository::PostRepository;
pub use user_repository::UserRepository;

pub struct RepositoryContainer {
    pub user: Box<dyn UserRepository>,
    pub post: Box<dyn PostRepository>,
}
