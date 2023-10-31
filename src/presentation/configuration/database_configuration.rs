use std::env;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

fn url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
}

pub async fn configure_pool() -> Pool<Postgres> {
    let connection = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url())
        .await;

    match connection {
        Ok(success) => success,
        Err(_) => panic!("Problem connect to database!"),
    }
}
