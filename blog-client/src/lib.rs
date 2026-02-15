use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;

mod http_client;
mod grpc_client;
mod error;

pub enum Transport {
    Http(String),
    Grpc(String)
}


pub struct BlogClient {
    transport: Transport,
    http_client: Option<HttpClient>,
    grpc_client: Option<GrpcClient>,
    token: Option<String>,
}