use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct FailureEntity {
    pub message: String,
}
