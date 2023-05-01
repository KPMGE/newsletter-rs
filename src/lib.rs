use actix_web::{get, Responder, HttpServer, App, HttpResponse};
use actix_web::dev::Server;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind(("localhost", 8080))?
    .run();

    Ok(server)
}
