use validator::Validate;

use crate::{domain::graph::Graph, error::ServiceError};

#[derive(Debug, Clone)]
pub struct ValidGraph(Graph);

impl TryFrom<&Graph> for ValidGraph {
    type Error = ServiceError;

    fn try_from(value: &Graph) -> Result<Self, Self::Error> {
        match value.validate() {
            Ok(_) => Ok(ValidGraph(value.to_owned())),
            Err(err) => Err(ServiceError::ValidationError(err.to_string())),
        }
    }
}

impl AsRef<Graph> for ValidGraph {
    fn as_ref(&self) -> &Graph {
        &self.0
    }
}
