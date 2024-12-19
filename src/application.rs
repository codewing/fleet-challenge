use std::{net::TcpListener, sync::Mutex};

use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::{
    domain::valid_graph::ValidGraph,
    routes::{health_controller, layout_controller, route_controller},
};

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

#[derive(Debug, Default)]
pub struct ValidationState {
    pub graph: Mutex<Option<ValidGraph>>,
}

pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let validation_state = web::Data::new(ValidationState {
        graph: Mutex::new(None),
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_controller::health_check)
            .service(
                web::scope("/v1")
                    .app_data(validation_state.clone())
                    .service(web::scope("/layout").service(layout_controller::validate))
                    .service(web::scope("/routing").service(route_controller::route)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
