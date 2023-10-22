use rocket::{get, http::Status, serde::json::Json};

use crate::application::models::response::Response;

#[get("/healthchecker")]
pub fn healthchecker_handler() -> Result<Json<Response>, Status> {
    Ok(Json(Response {
        status: "success".to_string(),
        message: "I want a coffee".to_string(),
    }))
}
