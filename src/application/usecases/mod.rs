pub mod create_coffee_use_case;
pub mod delete_coffee_use_case;
pub mod paginate_coffee_use_case;
pub mod search_coffee_use_case;
pub mod update_coffee_use_case;

use crate::domain::entities::{coffee_entity::CoffeeEntity, faiulure_entity::FailureEntity};

use super::models::{
    coffee_model::{CoffeeInModel, CoffeeOutModel},
    failure_model::FaiureOutModel,
    response_model::ResponseModel,
};

pub fn to_entity(model: CoffeeInModel) -> CoffeeEntity {
    CoffeeEntity {
        id: None,
        name: model.name,
        price: model.price,
        created_at: None,
        updated_at: None,
    }
}

pub fn to_model(entity: CoffeeEntity) -> CoffeeOutModel {
    CoffeeOutModel {
        id: entity.id.unwrap_or_default(),
        name: entity.name,
        price: entity.price,
        created_at: entity.created_at.unwrap_or_default(),
        updated_at: entity.updated_at.unwrap_or_default(),
    }
}

pub fn to_model_failure(entity: FailureEntity) -> ResponseModel<FaiureOutModel> {
    ResponseModel {
        status: "failure".to_string(),
        data: FaiureOutModel {
            message: entity.message,
        },
    }
}

pub fn to_model_success(entity: CoffeeEntity) -> ResponseModel<CoffeeOutModel> {
    ResponseModel {
        status: "success".to_string(),
        data: to_model(entity),
    }
}
