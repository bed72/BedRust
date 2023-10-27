use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::application::clients::database_client::DatabaseClient;
use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::entities::faiulure_entity::FailureEntity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;

pub struct CoffeeImplRepository {
    database: Box<dyn DatabaseClient<Pool<Postgres>>>,
}

impl CoffeeImplRepository {
    pub fn init(database: Box<dyn DatabaseClient<Pool<Postgres>>>) -> Self {
        CoffeeImplRepository { database }
    }
}

#[async_trait(?Send)]
impl CoffeeRepository for CoffeeImplRepository {
    async fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity> {
        let connection = self.database.connect().await;

        let value = sqlx::query(
            "INSERT INTO public.coffees (id, name, price, created_at, updated_at) VALUES(gen_random_uuid(), $1, $2, now(), now());",
        )
        .bind(data.name)
        .bind(data.price)
        .map(|response| {
            response
        })
        .fetch_one(&connection)
        .await;

        Ok(CoffeeEntity {
            id: None,
            name: "".to_string(),
            price: 0.0,
            created_at: None,
            updated_at: None,
        })
    }

    async fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity> {
        Ok(CoffeeEntity {
            id: None,
            name: "".to_string(),
            price: 0.0,
            created_at: None,
            updated_at: None,
        })
        // let mut connection = self.database.connect().await;

        // let schema: Result<CoffeeOutSchema, Error> =
        //     coffees
        //         .filter(id.eq(identifier))
        //         .get_result::<CoffeeOutSchema>(&mut connection);

        // schema.map(Self::to_entity).map_err(Self::to_failure)
    }

    async fn get_paginate(
        &self,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<CoffeeEntity>, FailureEntity> {
        let limit = limit.unwrap_or(10);
        let offset = (page.unwrap_or(1) - 1) * limit;
        let mut connection = self.database.connect().await;

        Ok(vec![])

        // let schemas: Result<Vec<CoffeeOutSchema>, Error> = coffees
        //     .limit(limit)
        //     .offset(offset)
        //     .select(CoffeeOutSchema::as_select())
        //     .load::<CoffeeOutSchema>(&mut connection);

        // schemas
        //     .map(|success| Self::to_entities(success))
        //     .map_err(|failure| FailureEntity {
        //         message: failure.to_string(),
        //     })
    }

    async fn delete(&self, identifier: Uuid) -> Result<usize, FailureEntity> {
        let coffee = self.get_by_id(identifier);
        let mut connection = self.database.connect().await;

        Ok(0)

        // if coffee.is_err() {
        //     return Err(coffee.err().unwrap());
        // }

        // let schema: Result<usize, Error> =
        //     diesel::delete(coffees.filter(id.eq(identifier))).execute(&mut connection);

        // schema.map_err(Self::to_failure)
    }

    async fn update(
        &self,
        identifier: Uuid,
        data: CoffeeEntity,
    ) -> Result<CoffeeEntity, FailureEntity> {
        let coffee = self.get_by_id(identifier);
        let mut connection = self.database.connect().await;

        Ok(CoffeeEntity {
            id: None,
            name: "".to_string(),
            price: 0.0,
            created_at: None,
            updated_at: None,
        })

        // if coffee.is_err() {
        //     return Err(coffee.err().unwrap());
        // }

        // let schema: Result<CoffeeOutSchema, Error> =
        //     diesel::update(coffees.filter(id.eq(identifier)))
        //         .set((name.eq(data.name), price.eq(data.price)))
        //         .get_result::<CoffeeOutSchema>(&mut connection);

        // schema.map(Self::to_entity).map_err(Self::to_failure)
    }
}
