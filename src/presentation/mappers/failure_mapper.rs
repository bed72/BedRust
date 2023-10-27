use crate::{
    application::models::failure_model::FailureOutModel,
    domain::entities::faiulure_entity::FailureEntity,
};

pub fn failure_to_model(entity: FailureEntity) -> FailureOutModel {
    FailureOutModel {
        message: entity.message,
    }
}
