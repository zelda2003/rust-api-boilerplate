mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();
    config::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    info!("🚀 Starting server on port {}", port);

    let db_pool = db::init_pool().await;
    match db_pool {
        Ok(pool) => {
            info!("✅ Database connected successfully!");
            HttpServer::new(move || App::new().app_data(pool.clone()).configure(routes::init))
                .bind(("127.0.0.1", port))?
                .run()
                .await
        }
        Err(err) => {
            error!("❌ Failed to connect to the database: {}", err);
            std::process::exit(1);
        }
    }
}
