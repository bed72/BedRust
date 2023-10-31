use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    application::models::{
        coffee_model::CoffeeInModel, failure_model::FailureOutModel, response_model::PaginatedModel,
    },
    domain::repositories::coffee_repository::CoffeeRepository,
    presentation::{
        configuration::coffee_state_configuration::CoffeeStateConfiguration,
        mappers::{
            coffee_mapper::{coffe_to_entity, coffee_to_model, coffees_to_model},
            failure_mapper::failure_to_model,
        },
    },
};

#[get("/{id}")]
async fn get_coffee_by_id_handler(
    state: web::Data<CoffeeStateConfiguration>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().json(invalid_id());
    }

    let response = state.repository.get_by_id(id.unwrap()).await;

    match response {
        Ok(success) => HttpResponse::Ok().json(coffee_to_model(success)),
        Err(failure) => HttpResponse::NotFound().json(failure_to_model(failure)),
    }
}

#[get("")]
async fn get_coffees_paginated_handler(
    state: web::Data<CoffeeStateConfiguration>,
    paginate: web::Query<PaginatedModel>,
) -> impl Responder {
    let response = state
        .repository
        .get_paginate(paginate.page.unwrap_or(1), paginate.limit.unwrap_or(10))
        .await;

    match response {
        Ok(success) => HttpResponse::Ok().json(coffees_to_model(success)),
        Err(failure) => HttpResponse::BadRequest().json(failure_to_model(failure)),
    }
}

#[post("")]
async fn create_coffee_handler(
    state: web::Data<CoffeeStateConfiguration>,
    payload: web::Json<CoffeeInModel>,
) -> impl Responder {
    let validate = payload.validate();

    if validate.is_err() {
        return HttpResponse::UnprocessableEntity().json(invalid_body(validate.err().unwrap()));
    }

    let response = state
        .repository
        .create(coffe_to_entity(payload.to_owned()))
        .await;

    match response {
        Ok(success) => HttpResponse::Ok().json(coffee_to_model(success)),
        Err(failure) => HttpResponse::BadRequest().json(failure_to_model(failure)),
    }
}

#[patch("/{id}")]
async fn update_coffee_handler(
    state: web::Data<CoffeeStateConfiguration>,
    path: web::Path<String>,
    payload: web::Json<CoffeeInModel>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().json(invalid_id());
    }

    let validate = payload.validate();

    if validate.is_err() {
        return HttpResponse::UnprocessableEntity().json(invalid_body(validate.err().unwrap()));
    }

    let response = state
        .repository
        .update(id.unwrap(), coffe_to_entity(payload.into_inner()))
        .await;

    match response {
        Ok(success) => HttpResponse::Ok().json(coffee_to_model(success)),
        Err(failure) => HttpResponse::NotFound().json(failure_to_model(failure)),
    }
}

#[delete("/{id}")]
async fn delete_coffee_handler(
    state: web::Data<CoffeeStateConfiguration>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().json(invalid_id());
    }

    let response = state.repository.delete(id.unwrap()).await;

    match response {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(failure) => HttpResponse::BadRequest().json(failure_to_model(failure)),
    }
}

fn invalid_id() -> FailureOutModel {
    FailureOutModel {
        message: "Invalid ID!".to_string(),
    }
}

fn invalid_body(error: ValidationErrors) -> FailureOutModel {
    let mut message = "Something goes wrong! ".to_string();

    for (_, values) in error.field_errors().iter() {
        for erro in values.iter() {
            message.push_str(erro.message.as_ref().unwrap());
        }
    }

    return FailureOutModel { message };
}

pub fn coffee_handler(configure: &mut web::ServiceConfig) {
    let factory = web::scope("/v1/api/coffee")
        .service(create_coffee_handler)
        .service(delete_coffee_handler)
        .service(update_coffee_handler)
        .service(get_coffee_by_id_handler)
        .service(get_coffees_paginated_handler);

    configure.service(factory);
}
