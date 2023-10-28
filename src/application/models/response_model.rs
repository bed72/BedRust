use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseModel<T> {
    pub status: String,
    pub data: T,
}

#[derive(Deserialize)]
pub struct PaginatedModel {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
