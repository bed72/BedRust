use crate::domain::entities::coffee_entity::CoffeeEntity;

pub trait CoffeeRepository {
    // fn delete(&self, id: Uuid);
    fn create(&self, data: CoffeeEntity) -> CoffeeEntity;
    // fn update(&self, id: Uuid, data: CoffeeEntity) -> CoffeeEntity;
    // fn get_by_id(&self, id: Uuid) -> CoffeeEntity;
    fn get_paginate(&self, page: Option<i64>, limit: Option<i64>) -> Vec<CoffeeEntity>;
}
