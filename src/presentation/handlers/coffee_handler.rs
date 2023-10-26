use rocket::{http::Status, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    application::models::{
        coffee_model::{CoffeeInModel, CoffeeOutModel},
        failure_model::FailureOutModel,
        response_model::ResponseModel,
    },
    domain::repositories::coffee_repository::CoffeeRepository,
    presentation::{
        container::coffee_container::CoffeeContainer,
        mappers::{
            coffee_mapper::{coffe_to_entity, coffee_to_response, coffees_to_response},
            failure_mapper::failure_to_response,
        },
    },
};

type CoffeeDeleteType = Result<Status, Custom<Json<ResponseModel<FailureOutModel>>>>;
type CoffeeType = Result<
    Custom<Json<ResponseModel<CoffeeOutModel>>>,
    Custom<Json<ResponseModel<FailureOutModel>>>,
>;
type CoffeePaginatedType = Result<
    Custom<Json<ResponseModel<Vec<CoffeeOutModel>>>>,
    Custom<Json<ResponseModel<FailureOutModel>>>,
>;

#[get("/coffee/<id>")]
pub fn get_coffee_by_id_handler(id: String, container: &State<CoffeeContainer>) -> CoffeeType {
    let parameter = Uuid::parse_str(&id);

    if parameter.is_err() {
        return Err(invalid_id());
    }

    let response = container.repository.get_by_id(parameter.unwrap());

    response
        .map(|success| coffee_to_response(Status::Ok, success))
        .map_err(|failure| failure_to_response(Status::BadRequest, failure))
}

#[get("/coffee?<page>&<limit>")]
pub fn get_all_coffees_handler(
    page: Option<i64>,
    limit: Option<i64>,
    container: &State<CoffeeContainer>,
) -> CoffeePaginatedType {
    let response = container.repository.get_paginate(page, limit);

    response
        .map(|success| coffees_to_response(Status::Ok, success))
        .map_err(|failure| failure_to_response(Status::BadRequest, failure))
}

#[post("/coffee", data = "<body>")]
pub fn create_coffee_handler(
    body: Json<CoffeeInModel>,
    container: &State<CoffeeContainer>,
) -> CoffeeType {
    let validate = body.clone().into_inner().validate();

    if validate.is_err() {
        return Err(invalid_body(validate));
    }

    let response = container
        .repository
        .create(coffe_to_entity(body.into_inner()));

    response
        .map(|success| coffee_to_response(Status::Ok, success))
        .map_err(|failure| failure_to_response(Status::BadRequest, failure))
}

#[patch("/coffee/<id>", data = "<body>")]
pub fn update_coffee_handler(
    id: String,
    body: Json<CoffeeInModel>,
    container: &State<CoffeeContainer>,
) -> CoffeeType {
    let parameter = Uuid::parse_str(&id);
    let validate = body.clone().into_inner().validate();

    if parameter.is_err() {
        return Err(invalid_id());
    }

    if validate.is_err() {
        return Err(invalid_body(validate));
    }

    let response = container
        .repository
        .update(parameter.unwrap(), coffe_to_entity(body.into_inner()));

    response
        .map(|success| coffee_to_response(Status::Ok, success))
        .map_err(|failure| failure_to_response(Status::BadRequest, failure))
}

#[delete("/coffee/<id>")]
pub fn delete_coffee_handler(id: String, container: &State<CoffeeContainer>) -> CoffeeDeleteType {
    let parameter = Uuid::parse_str(&id);

    if parameter.is_err() {
        return Err(invalid_id());
    }

    let response = container.repository.delete(parameter.unwrap());

    response
        .map(|_| Status::NoContent)
        .map_err(|failure| failure_to_response(Status::BadRequest, failure))
}

fn invalid_id() -> Custom<Json<ResponseModel<FailureOutModel>>> {
    Custom(
        Status::NotAcceptable,
        Json(ResponseModel {
            status: "failure".to_string(),
            data: FailureOutModel {
                message: "Invalid ID.".to_string(),
            },
        }),
    )
}

fn invalid_body(
    parameter: Result<(), ValidationErrors>,
) -> Custom<Json<ResponseModel<FailureOutModel>>> {
    let mut message = "Something goes wrong! ".to_string();

    for (_, values) in parameter.err().unwrap().field_errors().iter() {
        for error in values.iter() {
            message.push_str(error.message.as_ref().unwrap());
        }
    }

    return Custom(
        Status::UnprocessableEntity,
        Json(ResponseModel {
            status: "failure".to_string(),
            data: FailureOutModel { message },
        }),
    );
}
