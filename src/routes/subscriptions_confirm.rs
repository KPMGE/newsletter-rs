use actix_web::{get, web::Query, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    subscription_token: String
}

#[tracing::instrument(
    name = "confirm pending subscriber",
    skip(params)
)]
#[get("/subscriptions/confirm")]
pub async fn confirm(params: Query<Params>) -> HttpResponse {
    // HttpResponse::BadRequest().into()
    HttpResponse::Ok().finish()
}
