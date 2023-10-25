use uuid::Uuid;

use crate::domain::entities::{coffee_entity::CoffeeEntity, faiulure_entity::FailureEntity};

pub trait CoffeeRepository {
    fn delete(&self, identifier: Uuid) -> Result<usize, FailureEntity>;
    fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity>;
    fn update(&self, identifier: Uuid, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity>;
    fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity>;
    fn get_paginate(
        &self,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<CoffeeEntity>, FailureEntity>;
}
