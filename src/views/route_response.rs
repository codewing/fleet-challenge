use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Route {
    pub sequence: Vec<RouteStep>,
    pub distance: f32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteStep {
    NodeId(String),
    EdgeId(String),
}
