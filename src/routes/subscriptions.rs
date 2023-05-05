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

#[tracing::instrument(
    name  = "Adding new susbcriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email, 
        subscriber_name  = %form.name
    )
)]
#[post("/subscribe")]
pub async fn subscribe(form: Form<SubscribeFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name = "Saving new subscriber into the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &SubscribeFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
    r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await 
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
