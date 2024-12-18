mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    config::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let db_pool = db::init_pool()
        .await
        .expect("Database connection failed. Check DATABASE_URL in .env");

    HttpServer::new(move || App::new().app_data(db_pool.clone()).configure(routes::init))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
