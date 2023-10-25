use uuid::Uuid;

use crate::domain::repositories::coffee_repository::CoffeeRepository;
use crate::{application::models::CoffeeTypeModel, domain::usecases::use_case::UseCase};

use super::{to_model_failure, to_model_success};

pub struct SearchCoffeeUseCase;

impl SearchCoffeeUseCase {}

impl UseCase<Uuid, CoffeeTypeModel> for SearchCoffeeUseCase {
    fn execute(&self, repository: &impl CoffeeRepository, parameter: Uuid) -> CoffeeTypeModel {
        let response = repository.get_by_id(parameter);

        response.map(to_model_success).map_err(to_model_failure)
    }
}
