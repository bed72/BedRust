use handler::{
    create_coffee_handler, delete_coffee_handler, get_all_coffees_handler,
    get_coffee_by_id_handler, healthchecker_handler, update_coffee_handler,
};

#[macro_use]
extern crate rocket;

mod handler;
mod model;
mod response;

#[launch]
fn rocket() -> _ {
    let app = model::ApplicationState::init();
    rocket::build().manage(app).mount(
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
