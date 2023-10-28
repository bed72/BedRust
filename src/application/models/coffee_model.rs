use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct CoffeeInModel {
    #[validate(length(min = 4, message = "Name must be greater than 4 chars "))]
    pub name: String,
    #[validate(range(min = 2, max = 200, message = "Price must be between 2 and 200 "))]
    pub price: f64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CoffeeOutModel {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
