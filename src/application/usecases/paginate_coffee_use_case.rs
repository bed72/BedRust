use crate::{
    application::models::coffee_model::{CoffeeOutModel, CoffeesOutModel},
    domain::{
        entities::coffee_entity::CoffeeEntity, repositories::repository::Repository,
        usecases::use_case::UseCase,
    },
};

pub struct PaginateCoffeeUseCase;

impl UseCase<(Option<i64>, Option<i64>), CoffeesOutModel> for PaginateCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl Repository,
        data: (Option<i64>, Option<i64>),
    ) -> CoffeesOutModel {
        let (page, limit) = data;
        let response = repository.get_paginate(page, limit);

        Self::to_models(response)
    }
}

impl PaginateCoffeeUseCase {
    fn to_model(entity: CoffeeEntity) -> CoffeeOutModel {
        CoffeeOutModel {
            id: entity.id.unwrap_or_default(),
            name: entity.name,
            price: entity.price,
            created_at: entity.created_at.unwrap_or_default(),
            updated_at: entity.updated_at.unwrap_or_default(),
        }
    }

    fn to_models(entities: Vec<CoffeeEntity>) -> CoffeesOutModel {
        let coffees: Vec<CoffeeOutModel> = entities
            .iter()
            .map(|element| Self::to_model(element.clone()))
            .collect();

        CoffeesOutModel {
            quantity: coffees.len(),
            coffees,
        }
    }
}
