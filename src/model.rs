use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coffee {
    pub id: Option<String>,
    pub name: String,
    pub price: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct ApplicationState {
    pub coffee_database: Arc<Mutex<Vec<Coffee>>>,
}

impl ApplicationState {
    pub fn init() -> ApplicationState {
        ApplicationState {
            coffee_database: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoffee {
    pub name: Option<String>,
    pub price: Option<f64>,
}
