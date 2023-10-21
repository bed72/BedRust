use crate::models::coffee_model::CoffeeSelectable;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoffeeRequest {
    pub name: String,
    pub price: f64,
}

#[derive(Serialize)]
pub struct CoffeeResponse {
    pub coffee: CoffeeSelectable,
}

#[derive(Serialize)]
pub struct SingleCoffeeResponse {
    pub status: String,
    pub data: CoffeeResponse,
}

#[derive(Serialize)]
pub struct MultipleCoffeeResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<CoffeeSelectable>,
}
