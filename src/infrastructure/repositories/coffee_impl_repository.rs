use async_trait::async_trait;
use chrono::Local;
use sqlx::{types::uuid::Uuid, Error};

use crate::application::clients::database_client::DatabaseClient;
use crate::domain::entities::coffee_entity::CoffeeEntity;
use crate::domain::entities::faiulure_entity::FailureEntity;
use crate::domain::repositories::coffee_repository::CoffeeRepository;

#[derive(Debug, Clone)]
pub struct CoffeeImplRepository {
    database: DatabaseClient,
}

impl CoffeeImplRepository {
    pub fn init(database: DatabaseClient) -> Self {
        CoffeeImplRepository { database }
    }

    fn failure_to_entity(error: Error) -> FailureEntity {
        FailureEntity {
            message: error.to_string(),
        }
    }
}

#[async_trait(?Send)]
impl CoffeeRepository for CoffeeImplRepository {
    async fn create(&self, data: CoffeeEntity) -> Result<CoffeeEntity, FailureEntity> {
        let value = sqlx::query!(
                "INSERT INTO public.coffees (name, price) VALUES($1, $2) RETURNING id, name, price, created_at, updated_at;",
                data.name,
                data.price
            )
                .fetch_one(&self.database.pool)
                .await
                .map(|data| CoffeeEntity {
                    id: Some(data.id),
                    name: data.name,
                    price: data.price,
                    created_at: data.created_at,
                    updated_at: data.updated_at,
                });

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }

    async fn get_by_id(&self, identifier: Uuid) -> Result<CoffeeEntity, FailureEntity> {
        let value = sqlx::query!(
            "SELECT id, name, price, created_at, updated_at FROM public.coffees WHERE id = $1;",
            identifier,
        )
        .fetch_one(&self.database.pool)
        .await
        .map(|data| CoffeeEntity {
            id: Some(data.id),
            name: data.name,
            price: data.price,
            created_at: data.created_at,
            updated_at: data.updated_at,
        });

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

        let value = sqlx::query!(
            "SELECT id, name, price, created_at, updated_at FROM public.coffees LIMIT $1 OFFSET $2;",
            limit,
            offset,
        )
            .fetch_all(&self.database.pool)
            .await
            .map(|datas| datas
                .iter()
                .map(|data| CoffeeEntity {
                    id: Some(data.id),
                    name: data.name.clone(),
                    price: data.price,
                    created_at: data.created_at,
                    updated_at: data.updated_at,
                }
            )
            .collect()
        );

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }

    async fn delete(&self, identifier: Uuid) -> Result<(), FailureEntity> {
        let coffee = self.get_by_id(identifier).await;

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let value = sqlx::query!("DELETE FROM public.coffees WHERE id = $1;", identifier)
            .execute(&self.database.pool)
            .await;

        value.map(|_| ()).map_err(Self::failure_to_entity)
    }

    async fn update(
        &self,
        identifier: Uuid,
        data: CoffeeEntity,
    ) -> Result<CoffeeEntity, FailureEntity> {
        let coffee = self.get_by_id(identifier).await;

        if coffee.is_err() {
            return Err(coffee.err().unwrap());
        }

        let value = sqlx::query!(
            "UPDATE public.coffees SET name = $1, price = $2, updated_at = $3 WHERE id = $4 RETURNING id, name, price, created_at, updated_at;",
            data.name,
            data.price,
            Local::now(),
            identifier
        )
            .fetch_one(&self.database.pool)
            .await
            .map(|data| CoffeeEntity {
                id: Some(data.id),
                name: data.name,
                price: data.price,
                created_at: data.created_at,
                updated_at: data.updated_at,
            });

        value
            .map(|success| success)
            .map_err(Self::failure_to_entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Pool;
    use sqlx::Postgres;

    #[sqlx::test(fixtures("create.coffees.up"))]
    async fn test_get_paginate(pool: Pool<Postgres>) {
        let repository = CoffeeImplRepository::init(DatabaseClient { pool });

        let result = repository.get_paginate(1, 10).await;

        assert!(result.is_ok());

        match result {
            Ok(success) => {
                assert_eq!(success.len(), 1);
                assert_eq!(success[0].name, "Good Coffee");
                assert_eq!(success[0].price, 27.72);
            }
            Err(_) => {}
        }
    }
}
