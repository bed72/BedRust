use crate::{
    application::models::coffee_model::{CoffeeInModel, CoffeeOutModel},
    domain::entities::coffee_entity::CoffeeEntity,
};

pub fn coffe_to_entity(model: CoffeeInModel) -> CoffeeEntity {
    CoffeeEntity {
        id: None,
        name: model.name,
        price: model.price,
        created_at: None,
        updated_at: None,
    }
}

pub fn coffee_to_model(entity: CoffeeEntity) -> CoffeeOutModel {
    CoffeeOutModel {
        id: entity.id.unwrap_or_default(),
        name: entity.name,
        price: entity.price,
        created_at: entity.created_at.unwrap_or_default(),
        updated_at: entity.updated_at.unwrap_or_default(),
    }
}

pub fn coffees_to_model(entities: Vec<CoffeeEntity>) -> Vec<CoffeeOutModel> {
    let mut data: Vec<CoffeeOutModel> = Vec::with_capacity(entities.len());

    for entity in entities {
        data.push(coffee_to_model(entity))
    }

    data
}

// pub fn coffee_to_response(
//     status: Status,
//     entity: CoffeeEntity,
// ) -> Custom<Json<ResponseModel<CoffeeOutModel>>> {
//     Custom(
//         status,
//         Json(ResponseModel {
//             status: "success".to_string(),
//             data: coffee_to_model(entity),
//         }),
//     )
// }

//     Custom(
//         status,
//         Json(ResponseModel {
//             status: "success".to_string(),
//             data,
//         }),
//     )
// }
