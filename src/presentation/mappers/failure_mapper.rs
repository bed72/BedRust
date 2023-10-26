use rocket::{http::Status, response::status::Custom, serde::json::Json};

use crate::{
    application::models::{failure_model::FailureOutModel, response_model::ResponseModel},
    domain::entities::faiulure_entity::FailureEntity,
};

pub fn failure_to_model(entity: FailureEntity) -> FailureOutModel {
    FailureOutModel {
        message: entity.message,
    }
}

pub fn failure_to_response(
    status: Status,
    entity: FailureEntity,
) -> Custom<Json<ResponseModel<FailureOutModel>>> {
    Custom(
        status,
        Json(ResponseModel {
            status: "failure".to_string(),
            data: failure_to_model(entity),
        }),
    )
}
