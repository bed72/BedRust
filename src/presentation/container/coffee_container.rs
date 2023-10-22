use crate::{
    application::usecases::create_coffee_use_case::CreateCoffeeUseCase,
    infrastructure::repositories::coffee_repository::CoffeeRepository,
};

pub struct CoffeeContainer {
    pub repository: CoffeeRepository,
    pub use_case: CreateCoffeeUseCase,
}

impl CoffeeContainer {
    pub const INIT: CoffeeContainer = CoffeeContainer {
        repository: CoffeeRepository,
        use_case: CreateCoffeeUseCase,
    };
}
