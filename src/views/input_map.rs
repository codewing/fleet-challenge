use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InputMap {
    id: String,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: String,
    position: Position,
}

#[derive(Debug, Deserialize)]
struct Edge {
    id: String,
    source: String,
    sink: String,
}

#[derive(Debug, Deserialize)]
struct Position {
    x: f32,
    y: f32,
}
