use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;

use crate::infrastructure::databases::database::Database;
use crate::infrastructure::databases::postgres_database::PostgresDatabase;
use crate::infrastructure::schemas::coffee_schema::{CoffeeInSchema, CoffeeOutSchema};
use crate::infrastructure::schemas::schema::coffees::dsl::*;

pub struct CoffeeImplRepository;

impl CoffeeRepository for CoffeeImplRepository {
    fn create(&self, data: CoffeeEntity) -> CoffeeEntity {
        let connection = &mut PostgresDatabase::connect();

        let schema = diesel::insert_into(coffees)
            .values(&Self::to_schema(data))
            .returning(CoffeeOutSchema::as_returning())
            .get_result(connection)
            .expect("Failure to create new Coffee!");

        Self::to_entity(schema)
    }

    fn get_paginate(&self, page: Option<i64>, limit: Option<i64>) -> Vec<CoffeeEntity> {
        let connection = &mut PostgresDatabase::connect();

        let limit = limit.unwrap_or(10);
        let offset = (page.unwrap_or(1) - 1) * limit;

        let schema = coffees
            .limit(limit)
            .offset(offset)
            .select(CoffeeOutSchema::as_select())
            .load::<CoffeeOutSchema>(connection)
            .expect("Failure to the paginate Coffees!");

        Self::to_entities(schema)
    }
}

impl CoffeeImplRepository {
    fn to_schema(entity: CoffeeEntity) -> CoffeeInSchema {
        CoffeeInSchema {
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_entity(schema: CoffeeOutSchema) -> CoffeeEntity {
        CoffeeEntity {
            id: schema.id,
            name: schema.name,
            price: schema.price,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }

    fn to_entities(schema: Vec<CoffeeOutSchema>) -> Vec<CoffeeEntity> {
        schema
            .iter()
            .map(|element| Self::to_entity(element.clone()))
            .collect()
    }
}
