use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_graph", skip_on_field_errors = false))]
pub struct Graph {
    #[validate(length(min = 1))]
    id: String,
    #[validate(nested)]
    nodes: Vec<Node>,
    #[validate(nested)]
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct Node {
    #[validate(length(min = 1))]
    id: String,
    position: Position,
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Edge {
    #[validate(length(min = 1))]
    id: String,
    #[validate(length(min = 1))]
    source: String,
    #[validate(length(min = 1))]
    sink: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
}

fn validate_graph(value: &Graph) -> Result<(), ValidationError> {
    validate_driveway_connecting_two_existing_nodes(value)?;

    validate_node_degree(value)?;

    validate_node_connectivity(value)?;

    Ok(())
}

fn validate_node_connectivity(graph: &Graph) -> Result<(), ValidationError> {
    let mut visited_node_ids = HashSet::new();
    let mut border_node_ids = VecDeque::new();

    let first_node = graph
        .nodes
        .first()
        .ok_or(ValidationError::new("empty_graph"))?;
    border_node_ids.push_back(first_node.id.as_str());

    while !border_node_ids.is_empty() {
        let current_node_id = border_node_ids.pop_front().unwrap();

        visited_node_ids.insert(current_node_id);

        graph
            .edges
            .iter()
            .filter_map(|edge| {
                if edge.source == current_node_id {
                    Some(edge.sink.as_str())
                } else {
                    None
                }
            })
            .for_each(|node_id| {
                if !visited_node_ids.contains(node_id) {
                    border_node_ids.push_back(node_id);
                }
            });
    }

    if visited_node_ids.len() == graph.nodes.len() {
        Ok(())
    } else {
        Err(ValidationError::new("graph_has_partitions"))
    }
}

fn validate_node_degree(graph: &Graph) -> Result<(), ValidationError> {
    let mut node_degree_map: HashMap<&str, i32> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), 0))
        .collect();

    for edge in &graph.edges {
        match node_degree_map.get(edge.source.as_str()) {
            Some(degree) => node_degree_map.insert(edge.source.as_str(), degree + 1),
            None => return Err(ValidationError::new("missing_source_node")),
        };

        match node_degree_map.get(edge.sink.as_str()) {
            Some(degree) => node_degree_map.insert(edge.sink.as_str(), degree + 1),
            None => return Err(ValidationError::new("missing_sink_node")),
        };
    }

    for (_node, degree) in node_degree_map.iter() {
        if *degree < 2 {
            return Err(ValidationError::new("invalid_node_degree"));
        }
    }

    Ok(())
}

fn validate_driveway_connecting_two_existing_nodes(graph: &Graph) -> Result<(), ValidationError> {
    let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.as_str()).collect();

    for edge in &graph.edges {
        if !node_ids.contains(edge.source.as_str()) {
            return Err(ValidationError::new("missing_source_node"));
        }

        if !node_ids.contains(edge.sink.as_str()) {
            return Err(ValidationError::new("missing_sink_node"));
        }

        if edge.sink == edge.source {
            return Err(ValidationError::new("self_looping_edge"));
        }
    }

    Ok(())
}
