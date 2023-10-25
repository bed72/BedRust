use uuid::Uuid;

use crate::domain::repositories::coffee_repository::CoffeeRepository;
use crate::{
    application::models::{failure_model::FaiureOutModel, response_model::ResponseModel},
    domain::usecases::use_case::UseCase,
};

use super::to_model_failure;

pub struct DeleteCoffeeUseCase;

impl DeleteCoffeeUseCase {}

impl UseCase<Uuid, Result<usize, ResponseModel<FaiureOutModel>>> for DeleteCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl CoffeeRepository,
        parameter: Uuid,
    ) -> Result<usize, ResponseModel<FaiureOutModel>> {
        let response = repository.delete(parameter);

        response.map_err(to_model_failure)
    }
}
