use crate::{
    application::usecases::{
        create_coffee_use_case::CreateCoffeeUseCase,
        paginate_coffee_use_case::PaginateCoffeeUseCase,
    },
    infrastructure::repositories::coffee_repository::CoffeeRepository,
};

pub struct CoffeeContainer {
    pub repository: CoffeeRepository,
    pub create_use_case: CreateCoffeeUseCase,
    pub paginate_use_case: PaginateCoffeeUseCase,
}

impl CoffeeContainer {
    pub const INIT: CoffeeContainer = CoffeeContainer {
        repository: CoffeeRepository,
        create_use_case: CreateCoffeeUseCase,
        paginate_use_case: PaginateCoffeeUseCase,
    };
}
