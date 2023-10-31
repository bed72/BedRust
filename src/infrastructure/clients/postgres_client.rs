use sqlx::{Pool, Postgres};

use crate::application::clients::database_client::DatabaseClient;

impl DatabaseClient {
    pub fn init(pool: Pool<Postgres>) -> Self {
        DatabaseClient { pool }
    }
}
