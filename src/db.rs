use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL not found. Please add it to your .env file.");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
