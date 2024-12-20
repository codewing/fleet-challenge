use std::collections::{HashMap, HashSet};

use actix_web::{get, http::StatusCode, web, Responder};

use crate::{
    application::ValidationState,
    domain::graph::{Graph, Node},
    error::ServiceError,
    views::{
        route_request::RouteRequest,
        route_response::{Route, RouteStep},
        utils::to_response,
    },
};

#[get("/route")]
pub async fn route(
    route_request_query: web::Query<RouteRequest>,
    validation_state: web::Data<ValidationState>,
) -> impl Responder {
    let valid_graph = {
        match validation_state.graph.lock().unwrap().clone() {
            Some(valid_graph) => valid_graph,
            None => {
                return Err(ServiceError::InvalidStateError(
                    "No valid graph was validated by the validator endpoint yet".to_owned(),
                ))
            }
        }
    };

    let graph = valid_graph.as_ref();
    let route_request = route_request_query.into_inner();
    let start_node =
        graph
            .find_node(route_request.start.as_str())
            .ok_or(ServiceError::InvalidStateError(format!(
                "Start node not found {}",
                route_request.start
            )))?;
    let end_node =
        graph
            .find_node(route_request.goal.as_str())
            .ok_or(ServiceError::InvalidStateError(format!(
                "Goal node not found {}",
                route_request.goal
            )))?;

    let node_distances = gather_node_distances(graph, start_node, end_node);

    let route = build_route(graph, node_distances, start_node, end_node);

    Ok(to_response(route, StatusCode::OK))
}

fn gather_node_distances<'a>(
    graph: &'a Graph,
    start_node: &'a Node,
    end_node: &'a Node,
) -> HashMap<&'a str, (f32, &'a str)> {
    let mut node_distances = HashMap::new();
    node_distances.insert(start_node.id.as_str(), (0f32, ""));

    let node_map: HashMap<_, _> = graph
        .nodes
        .iter()
        .map(|x| (x.id.as_str(), x.position.clone()))
        .collect();

    let mut border_edges = HashSet::new();

    graph.edges.iter().for_each(|edge| {
        if edge.source == start_node.id {
            border_edges.insert(edge);
        }
    });

    while !node_distances.contains_key(end_node.id.as_str()) {
        let current_edge = border_edges
            .iter()
            .min_by(|x, y| {
                //didn't have the brain-power left yesterday to make it an a* but here it is :)
                let x_pos = node_map.get(x.sink.as_str()).unwrap();
                let y_pos = node_map.get(y.sink.as_str()).unwrap();

                let (x_dist_start, _) = node_distances.get(x.source.as_str()).unwrap();
                let (y_dist_start, _) = node_distances.get(y.source.as_str()).unwrap();
                let x_dist_end = x_pos.distance_to(&end_node.position);
                let y_dist_end = y_pos.distance_to(&end_node.position);

                (x_dist_start + x_dist_end)
                    .partial_cmp(&(y_dist_start + y_dist_end))
                    .expect("there shouldn't be any NaNs")
            })
            .unwrap()
            .to_owned();

        border_edges.remove(current_edge);

        let (prev_node_id, (prev_dist, _edge_id)) = node_distances
            .get_key_value(current_edge.source.as_str())
            .unwrap();

        let prev_node = graph.find_node(prev_node_id).unwrap();
        let next_node = graph.find_node(&current_edge.sink).unwrap();
        let dist = prev_node.position.distance_to(&next_node.position);

        let next_dist = prev_dist + dist;

        let should_insert = if let Some(node_distance_entry) =
            node_distances.get_key_value(next_node.id.as_str())
        {
            node_distance_entry.1 .0 > next_dist //existing longer path than new one
        } else {
            true
        };

        if should_insert {
            node_distances.insert(
                next_node.id.as_str(),
                (prev_dist + dist, current_edge.id.as_str()),
            );
        }

        graph.edges.iter().for_each(|edge| {
            if edge.source == next_node.id {
                border_edges.insert(edge);
            }
        });
    }

    node_distances
}

fn build_route(
    graph: &Graph,
    node_distances: HashMap<&str, (f32, &str)>,
    start_node: &Node,
    end_node: &Node,
) -> Route {
    let mut found_route = Route::default();
    let (dist, _) = node_distances.get(end_node.id.as_str()).unwrap();
    found_route.distance = *dist;
    let mut current_node = end_node;
    found_route
        .sequence
        .push(RouteStep::NodeId(end_node.id.clone()));

    while current_node.id != start_node.id {
        let (_, edge_id) = node_distances.get(current_node.id.as_str()).unwrap();

        let edge = graph
            .edges
            .iter()
            .find(|edge| edge.id.as_str() == *edge_id)
            .unwrap();

        found_route
            .sequence
            .push(RouteStep::EdgeId(edge.id.clone()));
        found_route
            .sequence
            .push(RouteStep::NodeId(edge.source.clone()));

        current_node = graph.find_node(edge.source.as_str()).unwrap();
    }

    found_route.sequence.reverse();
    found_route
}
