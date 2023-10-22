mod application;
mod domain;
mod infrastructure;
mod presentation;

use presentation::{
    container::coffee_container::CoffeeContainer,
    handlers::{
        coffee_handler::{create_coffee_handler, get_all_coffees_handler},
        healthchecker_hsndler::healthchecker_handler,
    },
};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let container = CoffeeContainer::INIT;
    rocket::build().manage(container).mount(
        "/v1/api",
        routes![
            healthchecker_handler,
            create_coffee_handler,
            get_all_coffees_handler,
            // update_coffee_handler,
            // delete_coffee_handler,
            // get_all_coffees_handler,
            // get_coffee_by_id_handler,
        ],
    )
}
