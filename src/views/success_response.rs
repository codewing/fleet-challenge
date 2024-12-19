use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

impl SuccessResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}
