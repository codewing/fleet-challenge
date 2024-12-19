use actix_web::{http::StatusCode, post, web, HttpResponse};

use crate::{
    application::ValidationState,
    domain::{graph::Graph, valid_graph::ValidGraph},
    error::ServiceError,
    views::{success_response::SuccessResponse, utils::to_response},
};

#[post("/validate")]
pub async fn validate(
    graph: web::Json<Graph>,
    validation_state: web::Data<ValidationState>,
) -> Result<HttpResponse, ServiceError> {
    let valid_graph = ValidGraph::try_from(&graph.into_inner())?;

    let mut graph = validation_state.graph.lock().unwrap();
    *graph = Some(valid_graph);

    Ok(to_response(
        SuccessResponse::new("Graph successfully validated!"),
        StatusCode::OK,
    ))
}
