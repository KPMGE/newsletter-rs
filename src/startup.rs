use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpServer, App};

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
