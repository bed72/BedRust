use dotenvy::dotenv;
use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{pg::PgConnection, Connection};

pub struct PostgresDatabase {
    pub connection: Arc<Mutex<PgConnection>>,
}

impl PostgresDatabase {
    pub fn init() -> PostgresDatabase {
        PostgresDatabase {
            connection: Arc::new(Mutex::new(connection())),
        }
    }
}

fn connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}
