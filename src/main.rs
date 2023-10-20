#[macro_use]
extern crate rocket;

mod databases;
mod handlers;
mod models;
mod responses;

use databases::memory_database::MemoryDatabase;
use handlers::coffee_handler::{
    create_coffee_handler, delete_coffee_handler, get_all_coffees_handler,
    get_coffee_by_id_handler, healthchecker_handler, update_coffee_handler,
};

#[launch]
fn rocket() -> _ {
    let database = MemoryDatabase::init();
    rocket::build().manage(database).mount(
        "/v1/api",
        routes![
            healthchecker_handler,
            create_coffee_handler,
            update_coffee_handler,
            delete_coffee_handler,
            get_all_coffees_handler,
            get_coffee_by_id_handler,
        ],
    )
}
