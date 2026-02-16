use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::error::BlogClientError;
use crate::{AuthResponse, Post, User};

pub struct HttpClient {
    base_url: Url,
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, BlogClientError> {
        let url = self.base_url.join("api/user/register").map_err(|e| {
            BlogClientError::InvalidRequest(format!("invalid base url: {e}"))
        })?;

        let body = RegisterRequest {
            username,
            email,
            password,
        };

        let resp = self.client.post(url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let user = resp.json::<UserResponse>().await?;
        Ok(User {
            id: user.id,
            username: user.username,
            email: None,
        })
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let url = self.base_url.join("api/auth/login").map_err(|e| {
            BlogClientError::InvalidRequest(format!("invalid base url: {e}"))
        })?;

        let body = LoginRequest { username, password };
        let resp = self.client.post(url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let auth = resp.json::<AuthResponseHttp>().await?;
        Ok(AuthResponse {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,
            user: Some(User {
                id: auth.user.id,
                username: auth.user.username,
                email: None,
            }),
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<AuthResponse, BlogClientError> {
        let url = self.base_url.join("api/auth/refresh").map_err(|e| {
            BlogClientError::InvalidRequest(format!("invalid base url: {e}"))
        })?;

        let body = RefreshRequest { refresh_token };
        let resp = self.client.post(url).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let auth = resp.json::<AuthResponseHttp>().await?;
        Ok(AuthResponse {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,
            user: Some(User {
                id: auth.user.id,
                username: auth.user.username,
                email: None,
            }),
        })
    }

    pub async fn create_post(
        &self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let url = self.base_url.join("api/posts").map_err(|e| {
            BlogClientError::InvalidRequest(format!("invalid base url: {e}"))
        })?;

        let body = CreatePostRequest { title, content };
        let resp = self
            .client
            .post(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let post = resp.json::<PostResponse>().await?;
        Ok(Post {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author_id: Some(post.author_id),
        })
    }

    pub async fn get_post(&self, token: &str, id: &str) -> Result<Post, BlogClientError> {
        let url = self
            .base_url
            .join(&format!("api/posts/{id}"))
            .map_err(|e| BlogClientError::InvalidRequest(format!("invalid base url: {e}")))?;

        let resp = self.client.get(url).bearer_auth(token).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let post = resp.json::<PostResponse>().await?;
        Ok(Post {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author_id: Some(post.author_id),
        })
    }

    pub async fn update_post(
        &self,
        token: &str,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let url = self
            .base_url
            .join(&format!("api/posts/{id}"))
            .map_err(|e| BlogClientError::InvalidRequest(format!("invalid base url: {e}")))?;

        let body = CreatePostRequest { title, content };
        let resp = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let post = resp.json::<PostResponse>().await?;
        Ok(Post {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author_id: Some(post.author_id),
        })
    }

    pub async fn delete_post(&self, token: &str, id: &str) -> Result<bool, BlogClientError> {
        let url = self
            .base_url
            .join(&format!("api/posts/{id}"))
            .map_err(|e| BlogClientError::InvalidRequest(format!("invalid base url: {e}")))?;

        let resp = self.client.delete(url).bearer_auth(token).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        Ok(true)
    }

    pub async fn list_posts(
        &self,
        token: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, BlogClientError> {
        if limit == 0 {
            return Err(BlogClientError::InvalidRequest(
                "limit must be > 0".to_string(),
            ));
        }

        let page = offset / limit + 1;
        let url = self
            .base_url
            .join(&format!("api/posts?page={page}&per_page={limit}"))
            .map_err(|e| BlogClientError::InvalidRequest(format!("invalid base url: {e}")))?;

        let resp = self.client.get(url).bearer_auth(token).send().await?;
        if !resp.status().is_success() {
            return Err(map_http_status(resp.status(), resp.text().await.unwrap_or_default()));
        }

        let posts = resp.json::<Vec<PostResponse>>().await?;
        Ok(posts
            .into_iter()
            .map(|post| Post {
                id: Some(post.id),
                title: post.title,
                content: post.content,
                author_id: Some(post.author_id),
            })
            .collect())
    }
}

#[derive(Serialize)]
struct RegisterRequest<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct CreatePostRequest<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct UserResponse {
    id: String,
    username: String,
}

#[derive(Deserialize)]
struct AuthResponseHttp {
    access_token: String,
    refresh_token: String,
    user: UserResponse,
}

#[derive(Deserialize)]
struct PostResponse {
    id: String,
    title: String,
    content: String,
    author_id: String,
}

#[derive(Serialize)]
struct RefreshRequest<'a> {
    refresh_token: &'a str,
}

fn map_http_status(status: reqwest::StatusCode, body: String) -> BlogClientError {
    match status {
        reqwest::StatusCode::UNAUTHORIZED => BlogClientError::Unauthorized,
        reqwest::StatusCode::NOT_FOUND => BlogClientError::NotFound,
        _ => BlogClientError::InvalidRequest(format!(
            "http status {} body {}",
            status.as_u16(),
            body
        )),
    }
}
