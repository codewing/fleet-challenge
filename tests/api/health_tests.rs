use crate::utils::app_utils;
use crate::utils::request_client::RequestClient;

#[tokio::test]
async fn health_check() {
    let app_config = app_utils::spawn_app().await;

    let client = RequestClient::new(&app_config.address);

    let response = client
        .get("/health-check")
        .await
        .expect("Failed to execute health-check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
