use serde::Serialize;

#[derive(Serialize)]
pub struct StatusRequest {
    pub message: String,
}
