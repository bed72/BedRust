use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::repositories::repository::Repository;
use crate::{
    application::models::coffee_model::{CoffeeInModel, CoffeeOutModel},
    domain::usecases::use_case::UseCase,
};

pub struct CreateCoffeeUseCase;

impl UseCase<CoffeeInModel, CoffeeOutModel> for CreateCoffeeUseCase {
    fn execute(&self, repository: &impl Repository, data: CoffeeInModel) -> CoffeeOutModel {
        let response = repository.create(Self::to_entity(data));

        Self::to_model(response)
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

    fn to_model(entity: CoffeeEntity) -> CoffeeOutModel {
        CoffeeOutModel {
            id: entity.id.unwrap_or_default(),
            name: entity.name,
            price: entity.price,
            created_at: entity.created_at.unwrap_or_default(),
            updated_at: entity.updated_at.unwrap_or_default(),
        }
    }
}
