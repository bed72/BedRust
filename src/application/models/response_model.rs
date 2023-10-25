use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseModel<T> {
    pub status: String,
    pub data: T,
}
