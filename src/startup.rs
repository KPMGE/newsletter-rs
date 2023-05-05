use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpServer, App, web::Data};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
