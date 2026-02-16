pub mod error;
pub mod grpc_client;
pub mod http_client;

use crate::error::BlogClientError;
use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Transport {
    Http(String),
    Grpc(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub author_id: Option<String>,
}

pub struct BlogClient {
    pub transport: Transport,
    pub http_client: Option<HttpClient>,
    pub grpc_client: Option<GrpcClient>,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
}

impl BlogClient {
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let (http_client, grpc_client) = match &transport {
            Transport::Http(base_url) => {
                let url = Url::parse(base_url).map_err(|e| {
                    BlogClientError::InvalidRequest(format!("invalid base url: {e}"))
                })?;
                (Some(HttpClient::new(url)), None)
            }
            Transport::Grpc(endpoint) => {
                let client = GrpcClient::connect(endpoint).await?;
                (None, Some(client))
            }
        };

        Ok(Self {
            transport,
            http_client,
            grpc_client,
            token: None,
            refresh_token: None,
        })
    }

    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn set_refresh_token(&mut self, token: impl Into<String>) {
        self.refresh_token = Some(token.into());
    }

    pub fn get_refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        match self.transport {
            Transport::Http(_) => {
                let http = self
                    .http_client
                    .as_ref()
                    .ok_or_else(|| BlogClientError::InvalidRequest("http client not initialized".to_string()))?;
                http.register(username, email, password).await?;
                let auth = self.login(username, password).await?;
                Ok(auth)
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                let auth = grpc.register(username, email, password).await?;
                self.set_token(auth.access_token.clone());
                self.set_refresh_token(auth.refresh_token.clone());
                Ok(auth)
            }
        }
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, BlogClientError> {
        let auth = match self.transport {
            Transport::Http(_) => {
                let http = self
                    .http_client
                    .as_ref()
                    .ok_or_else(|| BlogClientError::InvalidRequest("http client not initialized".to_string()))?;
                http.login(username, password).await?
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.login(username, password).await?
            }
        };

        self.set_token(auth.access_token.clone());
        self.set_refresh_token(auth.refresh_token.clone());
        Ok(auth)
    }

    pub async fn refresh(&mut self) -> Result<AuthResponse, BlogClientError> {
        let refresh = self
            .refresh_token
            .as_deref()
            .ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let http = self
                    .http_client
                    .as_ref()
                    .ok_or_else(|| BlogClientError::InvalidRequest("http client not initialized".to_string()))?;
                let auth = http.refresh(refresh).await?;
                self.set_token(auth.access_token.clone());
                self.set_refresh_token(auth.refresh_token.clone());
                Ok(auth)
            }
            Transport::Grpc(_) => Err(BlogClientError::InvalidRequest(
                "refresh is not supported for gRPC transport".to_string(),
            )),
        }
    }

    pub async fn create_post(&mut self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let result = {
                    let http = self
                        .http_client
                        .as_ref()
                        .ok_or_else(|| {
                            BlogClientError::InvalidRequest("http client not initialized".to_string())
                        })?;
                    http.create_post(token, title, content).await
                };
                match result {
                    Err(BlogClientError::Unauthorized) => {
                        self.refresh().await?;
                        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;
                        let http = self
                            .http_client
                            .as_ref()
                            .ok_or_else(|| {
                                BlogClientError::InvalidRequest("http client not initialized".to_string())
                            })?;
                        http.create_post(token, title, content).await
                    }
                    other => other,
                }
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.create_post(token, title, content).await
            }
        }
    }

    pub async fn get_post(&mut self, id: &str) -> Result<Post, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let result = {
                    let http = self
                        .http_client
                        .as_ref()
                        .ok_or_else(|| {
                            BlogClientError::InvalidRequest("http client not initialized".to_string())
                        })?;
                    http.get_post(token, id).await
                };
                match result {
                    Err(BlogClientError::Unauthorized) => {
                        self.refresh().await?;
                        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;
                        let http = self
                            .http_client
                            .as_ref()
                            .ok_or_else(|| {
                                BlogClientError::InvalidRequest("http client not initialized".to_string())
                            })?;
                        http.get_post(token, id).await
                    }
                    other => other,
                }
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.get_post(token, id).await
            }
        }
    }

    pub async fn update_post(
        &mut self,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let result = {
                    let http = self
                        .http_client
                        .as_ref()
                        .ok_or_else(|| {
                            BlogClientError::InvalidRequest("http client not initialized".to_string())
                        })?;
                    http.update_post(token, id, title, content).await
                };
                match result {
                    Err(BlogClientError::Unauthorized) => {
                        self.refresh().await?;
                        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;
                        let http = self
                            .http_client
                            .as_ref()
                            .ok_or_else(|| {
                                BlogClientError::InvalidRequest("http client not initialized".to_string())
                            })?;
                        http.update_post(token, id, title, content).await
                    }
                    other => other,
                }
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.update_post(token, id, title, content).await
            }
        }
    }

    pub async fn delete_post(&mut self, id: &str) -> Result<bool, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let result = {
                    let http = self
                        .http_client
                        .as_ref()
                        .ok_or_else(|| {
                            BlogClientError::InvalidRequest("http client not initialized".to_string())
                        })?;
                    http.delete_post(token, id).await
                };
                match result {
                    Err(BlogClientError::Unauthorized) => {
                        self.refresh().await?;
                        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;
                        let http = self
                            .http_client
                            .as_ref()
                            .ok_or_else(|| {
                                BlogClientError::InvalidRequest("http client not initialized".to_string())
                            })?;
                        http.delete_post(token, id).await
                    }
                    other => other,
                }
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.delete_post(token, id).await
            }
        }
    }

    pub async fn list_posts(
        &mut self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match self.transport {
            Transport::Http(_) => {
                let result = {
                    let http = self
                        .http_client
                        .as_ref()
                        .ok_or_else(|| {
                            BlogClientError::InvalidRequest("http client not initialized".to_string())
                        })?;
                    http.list_posts(token, limit, offset).await
                };
                match result {
                    Err(BlogClientError::Unauthorized) => {
                        self.refresh().await?;
                        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;
                        let http = self
                            .http_client
                            .as_ref()
                            .ok_or_else(|| {
                                BlogClientError::InvalidRequest("http client not initialized".to_string())
                            })?;
                        http.list_posts(token, limit, offset).await
                    }
                    other => other,
                }
            }
            Transport::Grpc(_) => {
                let grpc = self
                    .grpc_client
                    .as_mut()
                    .ok_or_else(|| BlogClientError::InvalidRequest("grpc client not initialized".to_string()))?;
                grpc.list_posts(token, limit, offset).await
            }
        }
    }
}
