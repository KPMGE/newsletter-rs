use serde::Deserialize;
use actix_web::{post, HttpResponse, web};
use actix_web::web::Form;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub name: String,
    pub email: String
}

#[post("/subscribe")]
pub async fn subscribe(form: Form<SubscribeFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("error when executing query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
