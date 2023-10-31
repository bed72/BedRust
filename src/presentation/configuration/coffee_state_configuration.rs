use sqlx::{Pool, Postgres};

use crate::{
    application::clients::database_client::DatabaseClient,
    infrastructure::repositories::coffee_impl_repository::CoffeeImplRepository,
};

#[derive(Debug, Clone)]
pub struct CoffeeStateConfiguration {
    pub repository: CoffeeImplRepository,
}

impl CoffeeStateConfiguration {
    pub fn init(pool: Pool<Postgres>) -> Self {
        let connection = DatabaseClient::init(pool);
        let repository = CoffeeImplRepository::init(connection);

        CoffeeStateConfiguration { repository }
    }
}
