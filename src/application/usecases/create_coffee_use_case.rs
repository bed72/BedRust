use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;
use crate::{
    application::models::coffee_model::{CoffeeInModel, CoffeeOutModel},
    domain::usecases::use_case::UseCase,
};

use super::to_model;

pub struct CreateCoffeeUseCase;

impl UseCase<CoffeeInModel, CoffeeOutModel> for CreateCoffeeUseCase {
    fn execute(&self, repository: &impl CoffeeRepository, data: CoffeeInModel) -> CoffeeOutModel {
        let response = repository.create(Self::to_entity(data));

        to_model(response)
    }
}

impl CreateCoffeeUseCase {
    fn to_entity(model: CoffeeInModel) -> CoffeeEntity {
        CoffeeEntity {
            id: None,
            name: model.name,
            price: model.price,
            created_at: None,
            updated_at: None,
        }
    }
}
