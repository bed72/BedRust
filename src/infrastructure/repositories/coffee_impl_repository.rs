use async_trait::async_trait;
use chrono::Local;
use sqlx::Row;
use sqlx::{
    postgres::PgRow,
    types::{chrono::NaiveDateTime, uuid::Uuid},
    Error, Pool, Postgres,
};

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

    fn failure_to_entity(error: Error) -> FailureEntity {
        FailureEntity {
            message: error.to_string(),
        }
    }

    fn coffe_to_entity(response: PgRow) -> CoffeeEntity {
        CoffeeEntity {
            id: Some(response.get::<Uuid, _>("id")),
            name: response.get::<String, _>("name"),
            price: response.get::<f64, _>("price"),
            created_at: Some(response.get::<NaiveDateTime, _>("created_at")),
            updated_at: Some(response.get::<NaiveDateTime, _>("updated_at")),
        }
    }

    fn coffes_to_entity(responses: PgRow) -> Vec<CoffeeEntity> {
        let entities: Vec<CoffeeEntity> = Vec::with_capacity(responses.len());

        // for response in responses {
        //     entities.push(Self::coffe_to_entity(response))
        // }

        entities
    }
}

#[async_trait(?Send)]
impl CoffeeRepository for CoffeeImplRepository {
    async fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity> {
        let connection = self.database.connect().await;

        let value = sqlx::query(
                "INSERT INTO public.coffees (name, price) VALUES($1, $2) RETURNING id, name, price, created_at, updated_at;"
            )
                .bind(data.name)
                .bind(data.price)
                .map(Self::coffe_to_entity)
                .fetch_one(&connection)
                .await;

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }

    async fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity> {
        let connection = self.database.connect().await;

        let value = sqlx::query(
            "SELECT id, name, price, created_at, updated_at FROM public.coffees WHERE id = '$1';",
        )
        .bind(identifier.to_string())
        .map(Self::coffe_to_entity)
        .fetch_one(&connection)
        .await;

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }

    async fn get_paginate(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<Vec<CoffeeEntity>, FailureEntity> {
        let offset = (page - 1) * limit;
        let connection = self.database.connect().await;

        let value = sqlx::query(
            "SELECT id, name, price, created_at, updated_at FROM public.coffees LIMIT $1 OFFSET = $2;",
        )
        .bind(limit)
        .bind(offset)
        .map(Self::coffes_to_entity)
        .fetch_one(&connection)
        .await;

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }

    async fn delete(&self, identifier: Uuid) -> Result<(), FailureEntity> {
        let connection = self.database.connect().await;
        let coffee = self.get_by_id(identifier).await;

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let value = sqlx::query("DELETE FROM public.coffees WHERE id = '$1';")
            .bind(identifier.to_string())
            .fetch_one(&connection)
            .await;

        value.map(|_| ()).map_err(Self::failure_to_entity)
    }

    async fn update(
        &self,
        identifier: Uuid,
        data: CoffeeEntity,
    ) -> Result<CoffeeEntity, FailureEntity> {
        let connection = self.database.connect().await;
        let coffee = self.get_by_id(identifier).await;

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let value = sqlx::query(
            "UPDATE public.coffees SET name = $1, price = $2, updated_at = $3 WHERE id = '$4' RETURNING id, name, price, created_at, updated_at;"
        )
            .bind(data.name)
            .bind(data.price)
            .bind(Local::now())
            .bind(identifier.to_string())
            .map(Self::coffe_to_entity)
            .fetch_one(&connection)
            .await;

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }
}
