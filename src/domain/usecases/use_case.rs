use crate::domain::repositories::coffee_repository::CoffeeRepository;

pub trait UseCase<IN, OUT> {
    fn execute(&self, repository: &impl CoffeeRepository, data: IN) -> OUT;
}
