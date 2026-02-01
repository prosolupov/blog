use crate::data::{PostRepository, UserRepository};

pub struct PostService {
    repo: Box<dyn PostRepository>,
}

impl PostService {
    pub fn new(repo: Box<dyn PostRepository>) -> Self {
        Self { repo }
    }
}
