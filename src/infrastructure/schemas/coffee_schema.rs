use chrono::NaiveDateTime;
use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable, Selectable},
};
use serde::Serialize;
use uuid::Uuid;

use super::schema::coffees;

#[derive(Insertable)]
#[diesel(table_name = coffees)]
#[diesel(check_for_backend(Pg))]
pub struct CoffeeInSchema {
    pub name: String,
    pub price: f64,
}

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = coffees)]
#[diesel(check_for_backend(Pg))]
pub struct CoffeeOutSchema {
    pub id: Option<Uuid>,
    pub name: String,
    pub price: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
