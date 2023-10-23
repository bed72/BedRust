use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct CoffeeInModel {
    pub name: String,
    pub price: f64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CoffeeOutModel {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CoffeesOutModel {
    pub quantity: usize,
    pub coffees: Vec<CoffeeOutModel>,
}
