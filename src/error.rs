#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("A validation error ocurred: {0}")]
    ValidationError(String),
}
