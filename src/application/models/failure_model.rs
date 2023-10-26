use serde::Serialize;

#[derive(Serialize)]
pub struct FailureOutModel {
    pub message: String,
}
