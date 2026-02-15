use crate::grpc_client::pb::blog_service_client::BlogServiceClient;
use crate::grpc_client::pb::LoginRequest;

pub mod pb {
    tonic::include_proto!("blog.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let mut client = BlogServiceClient::new(channel);

    let login_resp = client
        .login(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        }).await?;

    Ok(())
}