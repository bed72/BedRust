mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{middleware::Logger, web, App, HttpServer};
use presentation::{
    handlers::coffee_handler::{
        create_coffee_handler, delete_coffee_handler, get_coffee_by_id_handler,
        get_coffees_paginated_handler, update_coffee_handler,
    },
    states::coffee_state::CoffeeState,
};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(CoffeeState::init()))
            .service(create_coffee_handler)
            .service(delete_coffee_handler)
            .service(update_coffee_handler)
            .service(get_coffee_by_id_handler)
            .service(get_coffees_paginated_handler)
            .wrap(Logger::default())
            .wrap(Logger::new(
                "%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T",
            ))
    })
    .bind(("127.0.0.1", 7200))?
    .run()
    .await
}
