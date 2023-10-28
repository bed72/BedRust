use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use uuid::Uuid;
use validator::Validate;

use crate::{
    application::models::coffee_model::CoffeeInModel,
    domain::repositories::coffee_repository::CoffeeRepository,
    presentation::{
        mappers::{
            coffee_mapper::{coffe_to_entity, coffee_to_model, coffees_to_model},
            failure_mapper::failure_to_model,
        },
        states::coffee_state::CoffeeState,
    },
};

#[get("/v1/api/coffee/<id>")]
pub async fn get_coffee_by_id_handler(
    state: web::Data<CoffeeState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().body("Error");
    }

    let response = state.repository.get_by_id(id.unwrap()).await;

    if response.is_err() {
        return HttpResponse::NotFound().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::Ok().json(coffee_to_model(response.ok().unwrap()))
}

#[get("/v1/api/coffee?<page>&<limit>")]
pub async fn get_coffees_paginated_handler(
    state: web::Data<CoffeeState>,
    page: web::Query<i64>,
    limit: web::Query<i64>,
) -> impl Responder {
    let response = state.repository.get_paginate(page.0, limit.0).await;

    if response.is_err() {
        return HttpResponse::BadRequest().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::Ok().json(coffees_to_model(response.ok().unwrap()))
}

#[post("/v1/api/coffee")]
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

    if response.is_err() {
        return HttpResponse::BadRequest().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::Created().json(coffee_to_model(response.ok().unwrap()))
}

#[patch("/v1/api/coffee/<id>")]
pub async fn update_coffee_handler(
    state: web::Data<CoffeeState>,
    path: web::Path<String>,
    payload: web::Json<CoffeeInModel>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().body("Error");
    }

    let validate = payload.validate();

    if validate.is_err() {
        return HttpResponse::UnprocessableEntity().body("Error");
    }

    let response = state
        .repository
        .update(id.unwrap(), coffe_to_entity(payload.into_inner()))
        .await;

    if response.is_err() {
        return HttpResponse::BadRequest().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::Ok().json(coffee_to_model(response.ok().unwrap()))
}

#[delete("/v1/api/coffee/<id>")]
pub async fn delete_coffee_handler(
    state: web::Data<CoffeeState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path);

    if id.is_err() {
        return HttpResponse::Conflict().body("Error");
    }

    let response = state.repository.delete(id.unwrap()).await;

    if response.is_err() {
        return HttpResponse::BadRequest().json(failure_to_model(response.err().unwrap()));
    }

    HttpResponse::NoContent().finish()
}
