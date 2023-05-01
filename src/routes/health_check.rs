use actix_web::{get, Responder, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
