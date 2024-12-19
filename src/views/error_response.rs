use serde::{Deserialize, Serialize};

use crate::error::ServiceErrorDiscriminants;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: ServiceErrorDiscriminants,
    pub message: String,
}
