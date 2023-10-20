use crate::models;

use models::coffee_model::Coffee;
use std::sync::{Arc, Mutex};

pub struct MemoryDatabase {
    pub database: Arc<Mutex<Vec<Coffee>>>,
}

impl MemoryDatabase {
    pub fn init() -> MemoryDatabase {
        MemoryDatabase {
            database: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
