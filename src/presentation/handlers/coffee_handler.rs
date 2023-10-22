use rocket::{http::Status, response::status::Custom, serde::json::Json, State};

use crate::{
    application::models::{
        coffee_model::{CoffeeInModel, CoffeeOutModel, CoffeesOutModel},
        response::Response,
    },
    domain::usecases::use_case::UseCase,
    presentation::container::coffee_container::CoffeeContainer,
};

// #[get("/coffee/<identifier>")]
// pub fn get_coffee_by_id_handler(
//     identifier: i64,
//     data: &State<PostgresDatabase>,
// ) -> Result<Json<SingleCoffeeResponse>, Custom<Json<Response>>> {
//     let connection = &mut *data.connection.lock().unwrap();

//     let value: Vec<CoffeeSelectable> = coffees
//         .filter(id.eq(identifier))
//         .limit(1)
//         .select(CoffeeSelectable::as_select())
//         .load::<CoffeeSelectable>(connection)
//         .expect("Error");

//     if value.is_empty() {
//         return Err(failure(Status::NotFound));
//     }

//     Ok(success(value[0].clone()))
// }

#[get("/coffee?<page>&<limit>")]
pub fn get_all_coffees_handler(
    page: Option<i64>,
    limit: Option<i64>,
    data: &State<CoffeeContainer>,
) -> Result<Json<CoffeesOutModel>, Status> {
    let response = &data
        .paginate_use_case
        .execute(&data.repository, (page, limit));

    Ok(Json(response.to_owned()))
}

#[post("/coffee", data = "<body>")]
pub fn create_coffee_handler(
    body: Json<CoffeeInModel>,
    data: &State<CoffeeContainer>,
) -> Result<Json<CoffeeOutModel>, Custom<Json<Response>>> {
    let response = &data
        .create_use_case
        .execute(&data.repository, body.into_inner());

    Ok(Json(response.to_owned()))
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
// fn success(coffee: CoffeeSelectable) -> Json<SingleCoffeeResponse> {
//     let response = SingleCoffeeResponse {
//         status: "success".to_string(),
//         data: CoffeeResponse { coffee },
//     };

//     Json(response)
// }

// fn failure(status: Status) -> Custom<Json<Response>> {
//     let response = Response {
//         status: "fail".to_string(),
//         message: match status.code {
//             404 => "Coffee not found.".to_string(),
//             409 => "Coffee already exists".to_string(),
//             _ => "Opss, um erro aconteceu".to_string(),
//         },
//     };

//     Custom(status, Json(response))
// }
