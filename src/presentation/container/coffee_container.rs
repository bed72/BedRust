use crate::{
    application::usecases::{
        create_coffee_use_case::CreateCoffeeUseCase,
        paginate_coffee_use_case::PaginateCoffeeUseCase,
    },
    infrastructure::repositories::coffee_impl_repository::CoffeeImplRepository,
};

pub struct CoffeeContainer {
    pub repository: CoffeeImplRepository,
    pub create_use_case: CreateCoffeeUseCase,
    pub paginate_use_case: PaginateCoffeeUseCase,
}

impl CoffeeContainer {
    pub const INIT: CoffeeContainer = CoffeeContainer {
        repository: CoffeeImplRepository,
        create_use_case: CreateCoffeeUseCase,
        paginate_use_case: PaginateCoffeeUseCase,
    };
}
