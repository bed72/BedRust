use crate::infrastructure::{
    clients::postgres_client::PostgresClient,
    repositories::coffee_impl_repository::CoffeeImplRepository,
};
use dotenvy::dotenv;
use std::env;

pub struct CoffeeContainer {
    pub repository: CoffeeImplRepository,
}

impl CoffeeContainer {
    pub fn init() -> CoffeeContainer {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let connection = PostgresClient::init(url);
        let repository = CoffeeImplRepository::init(connection);

        CoffeeContainer { repository }
    }
}
