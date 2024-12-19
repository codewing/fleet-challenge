use arculus_fleet_manager::{
    application::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("fleet-manager".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let app = Application::build(8080)?;
    app.run_until_stopped().await?;

    Ok(())
}
