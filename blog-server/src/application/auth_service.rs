use crate::application::jwt_service::JwtService;
use crate::data::UserRepository;
use crate::domain::error::BlogError;
use crate::domain::jwt::AuthResponse;
use crate::domain::user::{UserAuthorization, UserResponse};
use crate::infrastructure::password::verify_password;
use std::sync::Arc;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repo,
            jwt_service,
        }
    }
    pub async fn login(&self, user: UserAuthorization) -> Result<AuthResponse, BlogError> {
        let respons_user = self
            .user_repo
            .get_user_by_username(user.username.clone())
            .await?;
        let check_password = verify_password(&*user.password, &*respons_user.password_hash).await;
        if check_password {
            let (access_token, refresh_token) = self
                .jwt_service
                .create_token(*respons_user.id.as_uuid(), respons_user.username.clone())
                .await?;
            Ok(AuthResponse {
                access_token,
                refresh_token,
                user: UserResponse {
                    id: respons_user.id.clone(),
                    username: respons_user.username,
                },
            })
        } else {
            Err(BlogError::InvalidCredential)
        }
    }

    pub async fn refresh(&self, refresh_token: String) -> Result<AuthResponse, BlogError> {
        let (access_token, refresh_token, user) = self.jwt_service.refresh(refresh_token).await?;
        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: UserResponse {
                id: user.id,
                username: user.username,
            },
        })
    }

    pub async fn logout(&self, refresh_token: String) -> Result<(), BlogError> {
        self.jwt_service.delete_token(refresh_token).await?;
        Ok(())
    }
}
