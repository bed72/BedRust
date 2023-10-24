use crate::domain::entities::coffee_entity::CoffeeEntity;

use super::models::coffee_model::CoffeeOutModel;

pub mod create_coffee_use_case;
pub mod paginate_coffee_use_case;

pub fn to_model(entity: CoffeeEntity) -> CoffeeOutModel {
    CoffeeOutModel {
        id: entity.id.unwrap_or_default(),
        name: entity.name,
        price: entity.price,
        created_at: entity.created_at.unwrap_or_default(),
        updated_at: entity.updated_at.unwrap_or_default(),
    }
}
