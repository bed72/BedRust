use crate::models;

use models::coffee_model::Coffee;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CoffeeResponse {
    pub coffee: Coffee,
}

#[derive(Debug, Serialize)]
pub struct SingleCoffeeResponse {
    pub status: String,
    pub data: CoffeeResponse,
}

#[derive(Debug, Serialize)]
pub struct MultipleCoffeeResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<Coffee>,
}
