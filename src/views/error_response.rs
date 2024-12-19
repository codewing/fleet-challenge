use serde::Serialize;

use crate::error::ServiceErrorDiscriminants;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_code: ServiceErrorDiscriminants,
    pub message: String,
}
