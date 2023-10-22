use crate::domain::repositories::repository::Repository;

pub trait UseCase<IN, OUT> {
    fn execute(&self, repository: &impl Repository, data: IN) -> OUT;
}
