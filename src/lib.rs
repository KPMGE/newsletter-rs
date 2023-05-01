use std::net::TcpListener;
use serde::Deserialize;
use actix_web::web::Form;
use actix_web::{get, Responder, HttpServer, App, HttpResponse, post};
use actix_web::dev::Server;

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

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub name: String,
    pub email: String
}

#[post("/subscribe")]
async fn subscribe(_form: Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
