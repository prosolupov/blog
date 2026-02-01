use crate::data::UserRepository;
use crate::domain::error::BlogError;
use crate::domain::user::{
    User, UserAuthorization, UserRegistration, UserResponse, UserResponseAuthentication,
};
use crate::infrastructure::password::{password_hash, verify_password};

pub struct UserService {
    repo: Box<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Box<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_new_user(
        &self,
        mut user: UserRegistration,
    ) -> Result<UserResponse, BlogError> {
        user.password = password_hash(&user.password)?;
        Ok(self.repo.create_user(user).await?)
    }

    pub async fn login(
        &self,
        user: UserAuthorization,
    ) -> Result<UserResponseAuthentication, BlogError> {
        let respons_user = self
            .repo
            .get_user_by_username(user.username.clone())
            .await?;
        let check_password = verify_password(&*user.password, &*respons_user.password_hash).await;
        if check_password {
            Ok(respons_user)
        } else {
            Err(BlogError::InvalidCredential)
        }
    }
}
