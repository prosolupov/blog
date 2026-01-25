use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().expect(".env file not found");
    let db_host = env::var("DB_HOST").expect("DATABASE_HOST not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME not set");
    let db_user = env::var("DB_USER").expect("DB_USER not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");

    let database_url = format!("postgres://{}:{}@{}:{}/{}",db_user,db_password,db_host,db_port,db_name);

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}