mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use crate::application::auth_service::AuthService;
use crate::application::jwt_service::JwtService;
use crate::application::post_service::PostService;
use crate::application::user_service::UserService;
use crate::data::RepositoryContainer;
use crate::data::post_repository::PostgresPostRepo;
use crate::data::user_repository::PostgresUserRepo;
use actix_web::{web, App, HttpServer};
use infrastructure::database::create_pool;
use presentation::{auth_handler, user_handler};
use actix_cors::Cors;
use crate::infrastructure::logging::init_tracing;
use crate::pb::blog_service_server::BlogServiceServer;
use crate::presentation::grpc_service::BlogGrpcService;
use crate::presentation::middleware::AutMiddleware;
use crate::presentation::post_handler;
use sqlx::{migrate, PgPool};
use std::sync::Arc;
use tonic::transport::Server;

pub mod pb {
    tonic::include_proto!("blog.v1");
}

fn create_repo_container(pool: &PgPool) -> RepositoryContainer {
    let user_repo_pg = Arc::new(PostgresUserRepo { pool: pool.clone() });
    let post_repo_pg = Arc::new(PostgresPostRepo { pool: pool.clone() });
    let post_jwt_pg = Arc::new(PostgresPostRepo { pool: pool.clone() });

    let repo_container = RepositoryContainer {
        user: user_repo_pg,
        post: post_repo_pg,
        jwt: post_jwt_pg,
    };

    repo_container
}

async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Применяет все миграции из папки migrations/
    migrate!("./migrations").run(pool).await?;
    Ok(())
}


async fn run_grpc_server(
    user: Arc<UserService>,
    post: Arc<PostService>,
    auth: Arc<AuthService>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:50051".parse()?;
    let svc = BlogGrpcService::new(auth, user, post);

    Server::builder()
        .add_service(BlogServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}

async fn run_http_server(
    user: Arc<UserService>,
    post: Arc<PostService>,
    auth: Arc<AuthService>,
) -> std::io::Result<()> {
    let user_data = web::Data::from(user);
    let post_data = web::Data::from(post);
    let auth_data = web::Data::from(auth);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(user_data.clone())
            .app_data(post_data.clone())
            .app_data(auth_data.clone())
            .configure(user_handler::init_routes)
            .configure(auth_handler::init_routes)
            .service(
                web::scope("/api")
                    .wrap(AutMiddleware)
                    .configure(post_handler::init_routes),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();


    let pool = create_pool().await.expect("Failed to create pool");
    let repo_container = create_repo_container(&pool);
    let jwt_service = Arc::new(JwtService::new(
        repo_container.jwt.clone(),
        repo_container.user.clone(),
    ));

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let user_service = Arc::new(UserService::new(repo_container.user.clone()));
    let post_service = Arc::new(PostService::new(repo_container.post.clone()));
    let auth_service = Arc::new(AuthService::new(repo_container.user.clone(), jwt_service.clone()));

    let grpc_user = user_service.clone();
    let grpc_post = post_service.clone();
    let grpc_auth = auth_service.clone();

    tokio::spawn(async move {
        if let Err(e) = run_grpc_server(grpc_user, grpc_post, grpc_auth).await {
            eprintln!("gRPC server error: {e}");
        }
    });

    run_http_server(user_service, post_service, auth_service).await
}
