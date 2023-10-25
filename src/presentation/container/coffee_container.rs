use crate::{
    application::usecases::{
        create_coffee_use_case::CreateCoffeeUseCase, delete_coffee_use_case::DeleteCoffeeUseCase,
        paginate_coffee_use_case::PaginateCoffeeUseCase,
        search_coffee_use_case::SearchCoffeeUseCase, update_coffee_use_case::UpdateCoffeeUseCase,
    },
    infrastructure::repositories::coffee_impl_repository::CoffeeImplRepository,
};

pub struct CoffeeContainer {
    pub repository: CoffeeImplRepository,
    pub create_use_case: CreateCoffeeUseCase,
    pub delete_use_case: DeleteCoffeeUseCase,
    pub update_use_case: UpdateCoffeeUseCase,
    pub search_use_case: SearchCoffeeUseCase,
    pub paginate_use_case: PaginateCoffeeUseCase,
}

impl CoffeeContainer {
    pub const INIT: CoffeeContainer = CoffeeContainer {
        repository: CoffeeImplRepository,
        create_use_case: CreateCoffeeUseCase,
        delete_use_case: DeleteCoffeeUseCase,
        update_use_case: UpdateCoffeeUseCase,
        search_use_case: SearchCoffeeUseCase,
        paginate_use_case: PaginateCoffeeUseCase,
    };
}
