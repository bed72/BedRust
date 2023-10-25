use crate::{
    application::models::{
        coffee_model::CoffeeOutModel, response_model::ResponseModel, CoffeesTypeModel,
    },
    domain::{
        entities::coffee_entity::CoffeeEntity, repositories::coffee_repository::CoffeeRepository,
        usecases::use_case::UseCase,
    },
};

use super::{to_model, to_model_failure};

pub struct PaginateCoffeeUseCase;

impl PaginateCoffeeUseCase {
    fn to_models_success(entities: Vec<CoffeeEntity>) -> ResponseModel<Vec<CoffeeOutModel>> {
        let mut models: Vec<CoffeeOutModel> = Vec::with_capacity(entities.len());

        for entity in entities {
            models.push(to_model(entity));
        }

        ResponseModel {
            status: "success".to_string(),
            data: models,
        }
    }
}

impl UseCase<(Option<i64>, Option<i64>), CoffeesTypeModel> for PaginateCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl CoffeeRepository,
        parameter: (Option<i64>, Option<i64>),
    ) -> CoffeesTypeModel {
        let (page, limit) = parameter;
        let response = repository.get_paginate(page, limit);

        response
            .map(Self::to_models_success)
            .map_err(to_model_failure)
    }
}
