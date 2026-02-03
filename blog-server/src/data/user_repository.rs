use crate::domain::error::BlogError;
use crate::domain::user::{User, UserResponseAuthentication};
use crate::domain::user::{UserLogin, UserRegistration, UserResponse, UserUpdate};
use async_trait::async_trait;
use sqlx::{ PgPool, query_as };

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: UserRegistration) -> Result<UserResponse, BlogError>;
    async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<UserResponse, BlogError>;
    async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<UserResponseAuthentication, BlogError>;
    // async fn update_user(&self, user: UserUpdate) -> Result<UserResponse, BlogError>;
}

#[derive(Clone)]
pub struct PostgresUserRepo {
    pub pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepo {
    async fn create_user(&self, user: UserRegistration) -> Result<UserResponse, BlogError> {
        let new_user = query_as!(
            UserResponse,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING username",
            user.username,
            user.email,
            user.password,
        )
            .fetch_one(&self.pool)
            .await;

        match new_user {
            Ok(new_user) => Ok(new_user),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code() == Some("23505".into()) {
                        return Err(BlogError::AlreadyExists(user.email));
                    }
                }
                Err(BlogError::DatabaseError(e))
            }
        }
    }

    async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<UserResponse, BlogError> {
        let user = sqlx::query_as!(UserResponse, "SELECT username FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<UserResponseAuthentication, BlogError> {
        let user = query_as!(
            UserResponseAuthentication,
            "SELECT id, username, password_hash FROM users WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // async fn update_user(&self, user: UserUpdate) -> Result<UserResponse, BlogError> {
    //     let user = sqlx::query_as!(
    //         UserResponse,
    //         "UPDATE users SET username = $1 WHERE id = $2 RETURNING username",
    //         user.username
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;
    //
    //     Ok(user)
    // }
}
