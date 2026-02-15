use tonic::transport::Channel;


pub mod pb {
    tonic::include_proto!("blog.v1");
}

pub struct GrpcClient {
    client: pb::blog_service_client::BlogServiceClient<Channel>,
}

impl GrpcClient {}