#[macro_use]
extern crate rocket;

mod databases;
mod handlers;
mod models;
mod responses;
mod schemas;

use databases::postgres_database::PostgresDatabase;
use handlers::coffee_handler::{
    create_coffee_handler, get_all_coffees_handler, get_coffee_by_id_handler, healthchecker_handler,
};

#[launch]
fn rocket() -> _ {
    let connection = PostgresDatabase::init();
    rocket::build().manage(connection).mount(
        "/v1/api",
        routes![
            healthchecker_handler,
            create_coffee_handler,
            // update_coffee_handler,
            // delete_coffee_handler,
            get_all_coffees_handler,
            get_coffee_by_id_handler,
        ],
    )
}
