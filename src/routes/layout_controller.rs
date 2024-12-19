use actix_web::{http::StatusCode, post, web, HttpResponse};

use crate::{
    domain::valid_graph::ValidGraph,
    error::ServiceError,
    views::{graph::Graph, utils::to_response},
};

#[post("/validate")]
pub async fn validate(graph: web::Json<Graph>) -> Result<HttpResponse, ServiceError> {
    let valid_graph = ValidGraph::try_from(&graph.into_inner())?;

    // store valid graph

    Ok(to_response("Good Graph!", StatusCode::OK))
}
