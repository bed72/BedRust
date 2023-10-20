use crate::model::Coffee;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

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
