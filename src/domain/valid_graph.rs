use validator::Validate;

use crate::{error::ServiceError, views::graph::Graph};

pub struct ValidGraph(Graph);

impl TryFrom<&Graph> for ValidGraph {
    type Error = ServiceError;

    fn try_from(value: &Graph) -> Result<Self, Self::Error> {
        match value.validate() {
            Ok(_) => Ok(ValidGraph {
                0: value.to_owned(),
            }),
            Err(err) => Err(ServiceError::ValidationError(err.to_string())),
        }
    }
}
