use actix_web::{http::StatusCode, ResponseError};
use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

use crate::views::{error_response::ErrorResponse, utils::to_response};

#[derive(Debug, thiserror::Error, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize), serde(rename_all = "snake_case"))]
pub enum ServiceError {
    #[error("A invalid state error ocurred: {0}")]
    InvalidStateError(String),

    #[error("A validation error ocurred: {0}")]
    ValidationError(String),

    #[error("An internal error ocurred: {0}")]
    InternalError(String),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::InvalidStateError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let response = ErrorResponse {
            error_code: self.into(),
            message: format!("{self}"),
        };

        to_response(response, self.status_code())
    }
}
