use arculus_fleet_manager::{
    application::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

const ANY_PORT: u16 = 0;

static TRACING: Lazy<()> = Lazy::new(|| {
    let filter_level = "info".to_string();
    let subscriber_name = "Test".to_string();

    let subscriber = get_subscriber(subscriber_name, filter_level, std::io::stdout);
    init_subscriber(subscriber);
});

pub struct TestAppConfig {
    pub address: String,
}

pub async fn spawn_app() -> TestAppConfig {
    Lazy::force(&TRACING);

    let application = Application::build(ANY_PORT).expect("Failed to build Application.");
    let address = format!("http://localhost:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestAppConfig { address }
}
