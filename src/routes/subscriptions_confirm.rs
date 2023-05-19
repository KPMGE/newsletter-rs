use actix_web::{get, HttpResponse};

#[tracing::instrument(
    name = "confirm pending subscriber"
)]
#[get("/subscriptions/confirm")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::BadRequest().into()
}
