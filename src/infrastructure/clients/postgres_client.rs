use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres};

use crate::application::clients::database_client::DatabaseClient;

pub struct PostgresClient {
    pub url: String,
}

impl PostgresClient {
    pub fn init(url: String) -> Self {
        PostgresClient { url }
    }
}

#[async_trait(?Send)]
impl DatabaseClient<Pool<Postgres>> for PostgresClient {
    async fn connect(&self) -> Pool<Postgres> {
        PgPool::connect(&self.url).await.unwrap()
    }
}
