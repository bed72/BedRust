mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{web, App, HttpServer};
use presentation::{
    handlers::coffee_handler::create_coffee_handler, states::coffee_state::CoffeeState,
};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(CoffeeState::init()))
            .service(create_coffee_handler)
    })
    .bind(("127.0.0.1", 7200))?
    .run()
    .await
}
