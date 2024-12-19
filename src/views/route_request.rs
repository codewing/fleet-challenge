use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RouteRequest {
    pub start: String,
    pub goal: String,
}
