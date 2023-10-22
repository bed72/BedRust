use chrono::{DateTime, Utc};
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CoffeesOutModel {
    pub quantity: usize,
    pub coffees: Vec<CoffeeOutModel>,
}
