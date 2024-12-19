use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::routes::{health_controller, layout_controller, route_controller};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn build(port: u16) -> Result<Application, std::io::Error> {
        let address = format!("localhost:{port}");
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();

        let server = run_server(listener)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_controller::health_check)
            .service(
                web::scope("/v1")
                    .service(web::scope("/layout").service(layout_controller::validate))
                    .service(web::scope("/routing").service(route_controller::route)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
