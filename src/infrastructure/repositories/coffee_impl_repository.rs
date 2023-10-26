use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::application::clients::database_client::DatabaseClient;
use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::entities::faiulure_entity::FailureEntity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;

use crate::infrastructure::clients::postgres_client::PostgresClient;
use crate::infrastructure::schemas::coffee_schema::{CoffeeInSchema, CoffeeOutSchema};
use crate::infrastructure::schemas::schema::coffees::dsl::*;

pub struct CoffeeImplRepository {
    database: PostgresClient,
}

impl CoffeeImplRepository {
    pub fn init(database: PostgresClient) -> Self {
        CoffeeImplRepository { database }
    }

    fn to_schema(entity: CoffeeEntity) -> CoffeeInSchema {
        CoffeeInSchema {
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_failure(failure: Error) -> FailureEntity {
        FailureEntity {
            message: failure.to_string(),
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

    fn to_entities(schemas: Vec<CoffeeOutSchema>) -> Vec<CoffeeEntity> {
        let mut entities: Vec<CoffeeEntity> = Vec::with_capacity(schemas.len());

        for schema in schemas {
            entities.push(Self::to_entity(schema));
        }

        entities
    }
}

impl CoffeeRepository for CoffeeImplRepository {
    fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity> {
        let mut connection = self
            .database
            .connect()
            .get()
            .expect("Couldn't get database connection from pool.");

        let schema: Result<CoffeeOutSchema, Error> = diesel::insert_into(coffees)
            .values(&Self::to_schema(data))
            .returning(CoffeeOutSchema::as_returning())
            .get_result::<CoffeeOutSchema>(&mut connection);

        schema.map(Self::to_entity).map_err(Self::to_failure)
    }

    fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity> {
        let mut connection = self
            .database
            .connect()
            .get()
            .expect("couldn't get db connection from pool");

        let schema: Result<CoffeeOutSchema, Error> =
            coffees
                .filter(id.eq(identifier))
                .get_result::<CoffeeOutSchema>(&mut connection);

        schema.map(Self::to_entity).map_err(Self::to_failure)
    }

    fn get_paginate(
        &self,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<CoffeeEntity>, FailureEntity> {
        let limit = limit.unwrap_or(10);
        let offset = (page.unwrap_or(1) - 1) * limit;
        let mut connection = self
            .database
            .connect()
            .get()
            .expect("couldn't get db connection from pool");

        let schemas: Result<Vec<CoffeeOutSchema>, Error> = coffees
            .limit(limit)
            .offset(offset)
            .select(CoffeeOutSchema::as_select())
            .load::<CoffeeOutSchema>(&mut connection);

        schemas
            .map(|success| Self::to_entities(success))
            .map_err(|failure| FailureEntity {
                message: failure.to_string(),
            })
    }

    fn delete(&self, identifier: Uuid) -> Result<usize, FailureEntity> {
        let coffee = self.get_by_id(identifier);
        let mut connection = self
            .database
            .connect()
            .get()
            .expect("couldn't get db connection from pool");

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let schema: Result<usize, Error> =
            diesel::delete(coffees.filter(id.eq(identifier))).execute(&mut connection);

        schema.map_err(Self::to_failure)
    }

    fn update(&self, identifier: Uuid, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity> {
        let coffee = self.get_by_id(identifier);
        let mut connection = self
            .database
            .connect()
            .get()
            .expect("couldn't get db connection from pool");

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let schema: Result<CoffeeOutSchema, Error> =
            diesel::update(coffees.filter(id.eq(identifier)))
                .set((name.eq(data.name), price.eq(data.price)))
                .get_result::<CoffeeOutSchema>(&mut connection);

        schema.map(Self::to_entity).map_err(Self::to_failure)
    }
}
