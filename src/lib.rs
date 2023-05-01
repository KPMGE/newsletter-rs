use actix_web::{get, Responder, HttpServer, App, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

