use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use validator::Validate;

use crate::{
    application::models::{
        coffee_model::{CoffeeInModel, CoffeeOutModel},
        failure_model::FailureOutModel,
    },
    domain::repositories::coffee_repository::CoffeeRepository,
    presentation::{
        mappers::{
            coffee_mapper::{coffe_to_entity, coffee_to_model},
            failure_mapper::failure_to_model,
        },
        states::coffee_state::CoffeeState,
    },
};

// type CoffeeDeleteType = Result<Status, Custom<Json<ResponseModel<FailureOutModel>>>>;
// type CoffeeType = Result<
//     Custom<Json<ResponseModel<CoffeeOutModel>>>,
//     Custom<Json<ResponseModel<FailureOutModel>>>,
// >;
// type CoffeePaginatedType = Result<
//     Custom<Json<ResponseModel<Vec<CoffeeOutModel>>>>,
//     Custom<Json<ResponseModel<FailureOutModel>>>,
// >;

// #[get("/coffee/<id>")]
// pub async fn get_coffee_by_id_handler(
//     id: String,
//     container: &State<CoffeeState>,
// ) -> CoffeeType {
//     let parameter = Uuid::parse_str(&id);

//     if parameter.is_err() {
//         return Err(invalid_id());
//     }

//     let response = container.repository.get_by_id(parameter.unwrap()).await;

//     response
//         .map(|success| coffee_to_response(Status::Ok, success))
//         .map_err(|failure| failure_to_response(Status::BadRequest, failure))
// }

// #[get("/coffee?<page>&<limit>")]
// pub async fn get_all_coffees_handler(
//     page: Option<i64>,
//     limit: Option<i64>,
//     container: &State<CoffeeState>,
// ) -> CoffeePaginatedType {
//     let response = container.repository.get_paginate(page, limit).await;

//     response
//         .map(|success| coffees_to_response(Status::Ok, success))
//         .map_err(|failure| failure_to_response(Status::BadRequest, failure))
// }
#[post("/coffee")]
pub async fn create_coffee_handler(
    state: web::Data<CoffeeState>,
    payload: web::Json<CoffeeInModel>,
) -> impl Responder {
    let validate = payload.validate();

    if validate.is_err() {
        return HttpResponse::UnprocessableEntity().body("Error");
    }

    let response = state
        .repository
        .create(coffe_to_entity(payload.to_owned()))
        .await;

    // response
    //     .map(|success| HttpResponse::Created().json(coffee_to_model(success)))
    //     .map_err(|failure| HttpResponse::BadRequest().json(failure_to_model(failure)))

    if response.is_err() {
        return HttpResponse::BadRequest().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::Created().json(coffee_to_model(response.ok().unwrap()))
}

// #[patch("/coffee/<id>", data = "<body>")]
// pub async fn update_coffee_handler(
//     id: String,
//     body: Json<CoffeeInModel>,
//     container: &State<CoffeeState>,
// ) -> CoffeeType {
//     let parameter = Uuid::parse_str(&id);
//     let validate = body.clone().into_inner().validate();

//     if parameter.is_err() {
//         return Err(invalid_id());
//     }

//     if validate.is_err() {
//         return Err(invalid_body(validate));
//     }

//     let response = container
//         .repository
//         .update(parameter.unwrap(), coffe_to_entity(body.into_inner()))
//         .await;

//     response
//         .map(|success| coffee_to_response(Status::Ok, success))
//         .map_err(|failure| failure_to_response(Status::BadRequest, failure))
// }

// #[delete("/coffee/<id>")]
// pub async fn delete_coffee_handler(
//     id: String,
//     container: &State<CoffeeState>,
// ) -> CoffeeDeleteType {
//     let parameter = Uuid::parse_str(&id);

//     if parameter.is_err() {
//         return Err(invalid_id());
//     }

//     let response = container.repository.delete(parameter.unwrap()).await;

//     response
//         .map(|_| Status::NoContent)
//         .map_err(|failure| failure_to_response(Status::BadRequest, failure))
// }

// fn invalid_id() -> Custom<Json<ResponseModel<FailureOutModel>>> {
//     Custom(
//         Status::NotAcceptable,
//         Json(ResponseModel {
//             status: "failure".to_string(),
//             data: FailureOutModel {
//                 message: "Invalid ID.".to_string(),
//             },
//         }),
//     )
// }

// fn invalid_body(
//     parameter: Result<(), ValidationErrors>,
// ) -> Custom<Json<ResponseModel<FailureOutModel>>> {
//     let mut message = "Something goes wrong! ".to_string();

//     for (_, values) in parameter.err().unwrap().field_errors().iter() {
//         for error in values.iter() {
//             message.push_str(error.message.as_ref().unwrap());
//         }
//     }

//     return Custom(
//         Status::UnprocessableEntity,
//         Json(ResponseModel {
//             status: "failure".to_string(),
//             data: FailureOutModel { message },
//         }),
//     );
// }
