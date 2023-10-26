mod application;
mod domain;
mod infrastructure;
mod presentation;

use presentation::{
    container::coffee_container::CoffeeContainer,
    handlers::coffee_handler::{
        create_coffee_handler, delete_coffee_handler, get_all_coffees_handler,
        get_coffee_by_id_handler, update_coffee_handler,
    },
};
use rocket::Config;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let configuration = Config {
        port: 7200,
        temp_dir: "/tmp/coffee".into(),
        ..Config::debug_default()
    };

    rocket::build()
        .manage(CoffeeContainer::init())
        .mount(
            "/v1/api",
            routes![
                create_coffee_handler,
                delete_coffee_handler,
                update_coffee_handler,
                get_all_coffees_handler,
                get_coffee_by_id_handler,
            ],
        )
        .configure(configuration)
}
