use arculus_fleet_manager::views::route_response::{Route, RouteStep};
use assertables::assert_approx_eq;

use crate::utils::request_client::RequestClient;
use crate::utils::{app_utils, graph_utils};

#[tokio::test]
async fn route_correct_test() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let valid_graph = graph_utils::load_graph("valid_map").unwrap();

    let layout_response = client
        .post("/v1/layout/validate", &valid_graph)
        .await
        .expect("Failed to execute layout validation request");

    assert!(layout_response.status().is_success());

    let from = "Node_BL";
    let to = "Node_TR";

    let url: String = format!("/v1/routing/route?start={from}&goal={to}");
    let routing_response = client
        .get(url.as_str())
        .await
        .expect("Failed to execute route request");

    assert!(routing_response.status().is_success());
    let route: Route = routing_response.json().await.unwrap();

    assert_eq!(route.sequence[0], RouteStep::NodeId("Node_BL".to_owned()));
    assert_eq!(route.sequence[1], RouteStep::EdgeId("BL_2_BC".to_owned()));
    assert_eq!(route.sequence[2], RouteStep::NodeId("Node_BC".to_owned()));
    assert_eq!(route.sequence[3], RouteStep::EdgeId("BC_2_TR".to_owned()));
    assert_eq!(route.sequence[4], RouteStep::NodeId("Node_TR".to_owned()));
}

#[tokio::test]
async fn route_distance_test() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let valid_graph = graph_utils::load_graph("valid_map").unwrap();

    let layout_response = client
        .post("/v1/layout/validate", &valid_graph)
        .await
        .expect("Failed to execute layout validation request");

    assert!(layout_response.status().is_success());

    let from = "Node_TL";
    let to = "Node_BR";

    let url: String = format!("/v1/routing/route?start={from}&goal={to}");
    let routing_response = client
        .get(url.as_str())
        .await
        .expect("Failed to execute route request");

    assert!(routing_response.status().is_success());
    let route: Route = routing_response.json().await.unwrap();

    assert_eq!(route.distance, 35f32);
}

#[tokio::test]
async fn route_finding_extended_test() {
    let app_config = app_utils::spawn_app().await;
    let client = RequestClient::new(&app_config.address);

    let valid_graph = graph_utils::load_graph("valid_map_extended").unwrap();

    let layout_response = client
        .post("/v1/layout/validate", &valid_graph)
        .await
        .expect("Failed to execute layout validation request");

    assert!(layout_response.status().is_success());

    let from = "Node_BL";
    let to = "Node_BRext";

    let url: String = format!("/v1/routing/route?start={from}&goal={to}");
    let routing_response = client
        .get(url.as_str())
        .await
        .expect("Failed to execute route request");

    assert!(routing_response.status().is_success());
    let route: Route = routing_response.json().await.unwrap();

    assert_approx_eq!(route.distance, 44.34869);
}
