use serde::Deserialize;
use actix_web::{post, HttpResponse, Responder};
use actix_web::web::Form;

#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub name: String,
    pub email: String
}

#[post("/subscribe")]
pub async fn subscribe(_form: Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
