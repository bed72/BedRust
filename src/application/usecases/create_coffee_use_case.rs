use crate::application::models::CoffeeTypeModel;
use crate::domain::repositories::coffee_repository::CoffeeRepository;
use crate::{
    application::models::coffee_model::CoffeeInModel, domain::usecases::use_case::UseCase,
};

use super::{to_entity, to_model_failure, to_model_success};

pub struct CreateCoffeeUseCase;

impl CreateCoffeeUseCase {}

impl UseCase<CoffeeInModel, CoffeeTypeModel> for CreateCoffeeUseCase {
    fn execute(
        &self,
        repository: &impl CoffeeRepository,
        parameter: CoffeeInModel,
    ) -> CoffeeTypeModel {
        let response = repository.create(to_entity(parameter));

        response.map(to_model_success).map_err(to_model_failure)
    }
}
