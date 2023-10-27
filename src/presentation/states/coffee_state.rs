use crate::infrastructure::{
    clients::postgres_client::PostgresClient,
    repositories::coffee_impl_repository::CoffeeImplRepository,
};
use dotenvy::dotenv;
use std::env;

pub struct CoffeeState {
    pub repository: CoffeeImplRepository,
}

impl CoffeeState {
    pub fn init() -> CoffeeState {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let connection = Box::new(PostgresClient::init(url));
        let repository = CoffeeImplRepository::init(connection);

        CoffeeState { repository }
    }
}
