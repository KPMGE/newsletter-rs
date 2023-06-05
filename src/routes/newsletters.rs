use actix_web::HttpResponse;
use actix_web::post;

#[post("/newsletters")]
pub async fn newsletters() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
