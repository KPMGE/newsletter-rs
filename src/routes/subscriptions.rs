use actix_web::web::Form;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::{NewSubscriber, SubscriberName, SubscriberEmail};

#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub name: String,
    pub email: String,
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
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name, 
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let new_subscriber = NewSubscriber {
        email,
        name
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new subscriber into the database", skip(pool))]
pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
