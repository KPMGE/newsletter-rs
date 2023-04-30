use actix_web::{get, Responder, HttpServer, App, HttpResponse};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}
