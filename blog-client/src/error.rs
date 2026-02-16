use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogClientError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("grpc status error: {0}")]
    Grpc(#[from] tonic::Status),
    #[error("grpc transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
}
