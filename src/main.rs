mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use presentation::states::coffee_state::CoffeeState;
use std::io::Result;

use crate::presentation::handlers::coffee_handler::coffee_configure;

#[actix_web::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(CoffeeState::init()))
            .configure(coffee_configure)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 7200))?
    .run()
    .await
}
