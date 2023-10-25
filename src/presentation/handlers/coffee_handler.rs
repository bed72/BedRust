use rocket::{http::Status, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    application::models::{
        coffee_model::{CoffeeInModel, CoffeeOutModel},
        failure_model::FaiureOutModel,
        response_model::ResponseModel,
    },
    domain::usecases::use_case::UseCase,
    presentation::container::coffee_container::CoffeeContainer,
};

type CoffeeDeleteType = Result<Status, Custom<Json<ResponseModel<FaiureOutModel>>>>;
type CoffeeType = Result<
    Custom<Json<ResponseModel<CoffeeOutModel>>>,
    Custom<Json<ResponseModel<FaiureOutModel>>>,
>;
type CoffeePaginatedType = Result<
    Custom<Json<ResponseModel<Vec<CoffeeOutModel>>>>,
    Custom<Json<ResponseModel<FaiureOutModel>>>,
>;

#[get("/coffee/<id>")]
pub fn get_coffee_by_id_handler(id: String, data: &State<CoffeeContainer>) -> CoffeeType {
    let parameter = Uuid::parse_str(&id);

    if parameter.is_err() {
        return Err(self::invalid_id());
    }

    let response = data
        .search_use_case
        .execute(&data.repository, parameter.unwrap());

    response
        .map(|success| Custom(Status::Ok, Json(success)))
        .map_err(|failure| Custom(Status::BadRequest, Json(failure)))
}

#[get("/coffee?<page>&<limit>")]
pub fn get_all_coffees_handler(
    page: Option<i64>,
    limit: Option<i64>,
    data: &State<CoffeeContainer>,
) -> CoffeePaginatedType {
    let response = data
        .paginate_use_case
        .execute(&data.repository, (page, limit));

    response
        .map(|success| Custom(Status::Ok, Json(success)))
        .map_err(|failure| Custom(Status::BadRequest, Json(failure)))
}

#[post("/coffee", data = "<body>")]
pub fn create_coffee_handler(
    body: Json<CoffeeInModel>,
    data: &State<CoffeeContainer>,
) -> CoffeeType {
    let validate = body.clone().into_inner().validate();

    if validate.is_err() {
        return Err(self::invalid_body(validate));
    }

    let response = data
        .create_use_case
        .execute(&data.repository, body.into_inner());

    response
        .map(|success| Custom(Status::Ok, Json(success)))
        .map_err(|failure| Custom(Status::BadRequest, Json(failure)))
}

#[patch("/coffee/<id>", data = "<body>")]
pub fn update_coffee_handler(
    id: String,
    body: Json<CoffeeInModel>,
    data: &State<CoffeeContainer>,
) -> CoffeeType {
    let parameter = Uuid::parse_str(&id);
    let validate = body.clone().into_inner().validate();

    if parameter.is_err() {
        return Err(self::invalid_id());
    }

    if validate.is_err() {
        return Err(self::invalid_body(validate));
    }

    let response = data
        .update_use_case
        .execute(&data.repository, (parameter.unwrap(), body.into_inner()));

    response
        .map(|success| Custom(Status::Ok, Json(success)))
        .map_err(|failure| Custom(Status::BadRequest, Json(failure)))
}

#[delete("/coffee/<id>")]
pub fn delete_coffee_handler(id: String, data: &State<CoffeeContainer>) -> CoffeeDeleteType {
    let parameter = Uuid::parse_str(&id);

    if parameter.is_err() {
        return Err(self::invalid_id());
    }

    let response = data
        .delete_use_case
        .execute(&data.repository, parameter.unwrap());

    response
        .map(|_| Status::NoContent)
        .map_err(|failure| Custom(Status::BadRequest, Json(failure)))
}

fn invalid_id() -> Custom<Json<ResponseModel<FaiureOutModel>>> {
    Custom(
        Status::NotAcceptable,
        Json(ResponseModel {
            status: "failure".to_string(),
            data: FaiureOutModel {
                message: "Invalid ID.".to_string(),
            },
        }),
    )
}

fn invalid_body(
    parameter: Result<(), ValidationErrors>,
) -> Custom<Json<ResponseModel<FaiureOutModel>>> {
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
            data: FaiureOutModel { message },
        }),
    );
}
