mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use crate::application::post_service::PostService;
use crate::application::user_service::UserService;
use crate::data::RepositoryContainer;
use crate::data::post_repository::PostgresPostRepo;
use crate::data::user_repository::PostgresUserRepo;
use actix_web::{App, HttpServer, web};
use infrastructure::database::create_pool;
use presentation::user_handler;
use sqlx::{PgPool, migrate};

fn create_repo_container(pool: &PgPool) -> RepositoryContainer {
    let user_repo_pg = Box::new(PostgresUserRepo { pool: pool.clone() });
    let post_repo_pg = Box::new(PostgresPostRepo { pool: pool.clone() });

    let repo_container = RepositoryContainer {
        user: user_repo_pg,
        post: post_repo_pg,
    };

    repo_container
}

async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Применяет все миграции из папки migrations/
    migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await.expect("Failed to create pool");
    let repo_container = create_repo_container(&pool);

    let user_service = web::Data::new(UserService::new(repo_container.user));
    let post_service = web::Data::new(PostService::new(repo_container.post));

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone())
            .app_data(post_service.clone())
            .configure(user_handler::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
