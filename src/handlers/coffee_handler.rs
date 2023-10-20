use crate::databases::memory_database::MemoryDatabase;
use crate::models::coffee_model::{Coffee, UpdateCoffee};
use crate::responses::coffee_response::{
    CoffeeResponse, MultipleCoffeeResponse, SingleCoffeeResponse,
};
use crate::responses::response::Response;

use chrono::prelude::Utc;
use rocket::{get, http::Status, post, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;

#[get("/healthchecker")]
pub async fn healthchecker_handler() -> Result<Json<Response>, Status> {
    Ok(Json(Response {
        status: "success".to_string(),
        message: "I want a coffee".to_string(),
    }))
}

#[get("/coffee?<page>&<limit>")]
pub fn get_all_coffees_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<MemoryDatabase>,
) -> Result<Json<MultipleCoffeeResponse>, Status> {
    let database = data.database.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let coffees: Vec<Coffee> = database
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let response = MultipleCoffeeResponse {
        status: "success".to_string(),
        results: coffees.len(),
        data: coffees,
    };

    Ok(Json(response))
}

#[get("/coffee/<id>")]
pub fn get_coffee_by_id_handler(
    id: String,
    data: &State<MemoryDatabase>,
) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
    let database = data.database.lock().unwrap();

    for coffee in database.iter() {
        if coffee.id == Some(id.to_owned()) {
            return Ok(success(coffee.clone()));
        }
    }

    Err(failure(Status::NotFound))
}

#[post("/coffee", data = "<body>")]
pub fn create_coffee_handler(
    mut body: Json<Coffee>,
    data: &State<MemoryDatabase>,
) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
    let mut database = data.database.lock().unwrap();

    for coffee in database.iter() {
        if coffee.name == body.name {
            return Err(failure(Status::Conflict));
        }
    }

    let id = Uuid::new_v4();
    let date = Utc::now();

    body.id = Some(id.to_string());
    body.created_at = Some(date);
    body.updated_at = Some(date);

    let coffee = body.to_owned();

    database.push(body.into_inner());

    Ok(success(coffee.into_inner().clone()))
}

#[delete("/coffee/<id>")]
pub async fn delete_coffee_handler(
    id: String,
    data: &State<MemoryDatabase>,
) -> Result<Status, Custom<Json<Response>>> {
    let mut database = data.database.lock().unwrap();

    for coffee in database.iter() {
        if coffee.id == Some(id.clone()) {
            database.retain(|coffee| coffee.id != Some(id.to_owned()));

            return Ok(Status::NoContent);
        }
    }

    Err(failure(Status::NotFound))
}

#[patch("/coffee/<id>", data = "<body>")]
pub fn update_coffee_handler(
    id: String,
    body: Json<UpdateCoffee>,
    data: &State<MemoryDatabase>,
) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
    let mut database = data.database.lock().unwrap();

    for coffee in database.iter_mut() {
        if coffee.id == Some(id.clone()) {
            let date = Utc::now();
            let name = body.name.to_owned().unwrap_or(coffee.name.to_owned());
            let price = body.price.to_owned().unwrap_or(coffee.price.to_owned());

            let payload = Coffee {
                id: coffee.id.to_owned(),
                name: if !name.is_empty() {
                    name
                } else {
                    coffee.name.to_owned()
                },
                price: if price != 0.0 {
                    price
                } else {
                    coffee.price.to_owned()
                },
                created_at: coffee.created_at,
                updated_at: Some(date),
            };

            *coffee = payload;

            return Ok(success(coffee.clone()));
        }
    }

    Err(failure(Status::NotFound))
}

fn success(coffee: Coffee) -> Json<SingleCoffeeResponse> {
    let response = SingleCoffeeResponse {
        status: "success".to_string(),
        data: CoffeeResponse { coffee },
    };

    Json(response)
}

fn failure(status: Status) -> Custom<Json<Response>> {
    let response = Response {
        status: "fail".to_string(),
        message: match status.code {
            404 => "Coffee not found.".to_string(),
            409 => "Coffee already exists".to_string(),
            _ => "Opss, um erro aconteceu".to_string(),
        },
    };

    Custom(status, Json(response))
}
