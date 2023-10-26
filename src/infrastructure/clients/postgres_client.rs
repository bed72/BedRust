use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::application::clients::database_client::DatabaseClient;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PostgresClient {
    pub url: String,
}

impl PostgresClient {
    pub fn init(url: String) -> Self {
        PostgresClient { url }
    }
}

impl DatabaseClient<DbPool> for PostgresClient {
    fn connect(&self) -> DbPool {
        let manager = ConnectionManager::<PgConnection>::new(&self.url);

        Pool::new(manager).unwrap()
    }
}
