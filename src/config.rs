use dotenvy::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
}
