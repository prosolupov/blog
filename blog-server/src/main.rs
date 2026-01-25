mod infrastructure;

use actix_web::{App, HttpServer};
use sqlx::{PgPool, migrate};
use infrastructure::database::create_pool;
async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Применяет все миграции из папки migrations/
    migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await.expect("Failed to create pool");

    run_migrations(&pool).await.expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
    })
    .bind("127.0.0.1:8080")?.run().await
}
