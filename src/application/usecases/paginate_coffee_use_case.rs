use crate::{
    application::models::coffee_model::{CoffeeOutModel, CoffeesOutModel},
    domain::{
        entities::coffee_entity::CoffeeEntity, repositories::coffee_repository::CoffeeRepository,
        usecases::use_case::UseCase,
    },
};

use super::to_model;

pub struct PaginateCoffeeUseCase;

impl UseCase<(Option<i64>, Option<i64>), CoffeesOutModel> for PaginateCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl CoffeeRepository,
        data: (Option<i64>, Option<i64>),
    ) -> CoffeesOutModel {
        let (page, limit) = data;
        let response = repository.get_paginate(page, limit);

        Self::to_models(response)
    }
}

impl PaginateCoffeeUseCase {
    fn to_models(entities: Vec<CoffeeEntity>) -> CoffeesOutModel {
        let coffees: Vec<CoffeeOutModel> = entities
            .iter()
            .map(|element| to_model(element.clone()))
            .collect();

        CoffeesOutModel {
            quantity: coffees.len(),
            coffees,
        }
    }
}
