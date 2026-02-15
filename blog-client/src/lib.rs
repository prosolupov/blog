mod http_client;
mod grpc_client;
mod error;

pub enum Transport {
    Http(String),
    Grpc(String)
}


pub struct BlogClient {
    transport: Transport,
    http_client: Option<>,
    grpc_client: Option<>,
    token: Option<String>,
}