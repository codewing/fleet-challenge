use crate::utils::request_client::RequestClient;
use crate::utils::{app_utils, graph_utils};

#[tokio::test]
async fn validate_valid_map() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let valid_graph = graph_utils::load_graph("valid_map").unwrap();

    let response = client
        .post("/v1/layout/validate", &valid_graph)
        .await
        .expect("Failed to execute layout validation request");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn validate_invalid_edge_in_map() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let invalid_node_in_edge = graph_utils::load_graph("invalid_node_in_edge").unwrap();

    let response = client
        .post("/v1/layout/validate", &invalid_node_in_edge)
        .await
        .expect("Failed to execute layout validation request");

    assert!(!response.status().is_success());
}

#[tokio::test]
async fn validate_invalid_node_edge_count_map() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let invalid_node_in_edge = graph_utils::load_graph("invalid_node_edge_count").unwrap();

    let response = client
        .post("/v1/layout/validate", &invalid_node_in_edge)
        .await
        .expect("Failed to execute layout validation request");

    assert!(!response.status().is_success());
}

#[tokio::test]
async fn validate_partitioned_map() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let partitioned_graph = graph_utils::load_graph("partitioned_graph").unwrap();

    let response = client
        .post("/v1/layout/validate", &partitioned_graph)
        .await
        .expect("Failed to execute layout validation request");

    assert!(!response.status().is_success());
}
