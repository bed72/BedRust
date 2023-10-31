use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DatabaseClient {
    pub pool: Pool<Postgres>,
}
