use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable, Selectable},
};
use serde::Serialize;

use crate::schemas::schema::coffees;

#[derive(Insertable)]
#[diesel(table_name = coffees)]
#[diesel(check_for_backend(Pg))]
pub struct CoffeeInsert {
    pub name: String,
    pub price: f64,
}

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = coffees)]
#[diesel(check_for_backend(Pg))]
pub struct CoffeeSelectable {
    pub id: i64,
    pub name: String,
    pub price: f64,
}
