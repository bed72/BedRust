use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable, Selectable},
};
use serde::Serialize;

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
    // pub id: None,
    pub name: String,
    pub price: f64,
    // pub created_at: Option<DateTime>,
    // pub updated_at: Option<DateTime>,
}
