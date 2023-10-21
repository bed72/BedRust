use crate::databases::postgres_database::PostgresDatabase;
use crate::models::coffee_model::{CoffeeInsert, CoffeeSelectable};
use crate::responses::coffee_response::{
    CoffeeRequest, CoffeeResponse, MultipleCoffeeResponse, SingleCoffeeResponse,
};
use crate::responses::response::Response;
use crate::schemas::schema::coffees::dsl::*;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, http::Status, response::status::Custom, serde::json::Json, State};

#[get("/healthchecker")]
pub async fn healthchecker_handler() -> Result<Json<Response>, Status> {
    Ok(Json(Response {
        status: "success".to_string(),
        message: "I want a coffee".to_string(),
    }))
}

#[get("/coffee?<page>&<limit>")]
pub fn get_all_coffees_handler(
    page: Option<i64>,
    limit: Option<i64>,
    data: &State<PostgresDatabase>,
) -> Result<Json<MultipleCoffeeResponse>, Status> {
    let connection = &mut *data.connection.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let value = coffees
        .limit(limit)
        .offset(offset)
        .select(CoffeeSelectable::as_select())
        .load::<CoffeeSelectable>(connection)
        .expect("Error");

    let response = MultipleCoffeeResponse {
        status: "success".to_string(),
        results: value.len(),
        data: value,
    };

    Ok(Json(response))
}

#[get("/coffee/<identifier>")]
pub fn get_coffee_by_id_handler(
    identifier: i64,
    data: &State<PostgresDatabase>,
) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
    let connection = &mut *data.connection.lock().unwrap();

    let value: Vec<CoffeeSelectable> = coffees
        .filter(id.eq(identifier))
        .limit(1)
        .select(CoffeeSelectable::as_select())
        .load::<CoffeeSelectable>(connection)
        .expect("Error");

    if value.is_empty() {
        return Err(failure(Status::NotFound));
    }

    Ok(success(value[0].clone()))
}

#[post("/coffee", data = "<body>")]
pub fn create_coffee_handler(
    body: Json<CoffeeRequest>,
    data: &State<PostgresDatabase>,
) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
    let connection = &mut *data.connection.lock().unwrap();

    let data = body.into_inner();
    let model = CoffeeInsert {
        name: data.name,
        price: data.price,
    };

    let value = diesel::insert_into(coffees)
        .values(&model)
        .returning(CoffeeSelectable::as_returning())
        .get_result(connection)
        .expect("Error");

    Ok(success(value.clone()))
}
/*
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
            // let date = Utc::now();
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
                // created_at: coffee.created_at,
                // updated_at: Some(date),
            };

            *coffee = payload;

            return Ok(success(coffee.clone()));
        }
    }

    Err(failure(Status::NotFound))
}
*/
fn success(coffee: CoffeeSelectable) -> Json<SingleCoffeeResponse> {
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
