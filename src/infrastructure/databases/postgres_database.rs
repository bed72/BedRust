use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

use super::database::Database;

pub struct PostgresDatabase;

impl Database<PgConnection> for PostgresDatabase {
    fn connect() -> PgConnection {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
    }
}
