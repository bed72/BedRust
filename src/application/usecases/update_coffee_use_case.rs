use crate::application::usecases::to_entity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;
use crate::{
    application::models::{coffee_model::CoffeeInModel, CoffeeTypeModel},
    domain::usecases::use_case::UseCase,
};
use uuid::Uuid;

use super::{to_model_failure, to_model_success};

pub struct UpdateCoffeeUseCase;

impl UpdateCoffeeUseCase {}

impl UseCase<(Uuid, CoffeeInModel), CoffeeTypeModel> for UpdateCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl CoffeeRepository,
        parameter: (Uuid, CoffeeInModel),
    ) -> CoffeeTypeModel {
        let response = repository.update(parameter.0, to_entity(parameter.1));

        response.map(to_model_success).map_err(to_model_failure)
    }
}
