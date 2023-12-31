use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{coffee_entity::CoffeeEntity, faiulure_entity::FailureEntity};

#[async_trait(?Send)]
pub trait CoffeeRepository {
    async fn delete(&self, identifier: Uuid) -> Result<(), FailureEntity>;
    async fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity>;
    async fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity>;
    async fn get_paginate(&self, page: i64, limit: i64)
        -> Result<Vec<CoffeeEntity>, FailureEntity>;
    async fn update(
        &self,
        identifier: Uuid,
        data: CoffeeEntity,
    ) -> Result<CoffeeEntity, FailureEntity>;
}
