use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coffee {
    pub id: Option<String>,
    pub name: String,
    pub price: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoffee {
    pub name: Option<String>,
    pub price: Option<f64>,
}
