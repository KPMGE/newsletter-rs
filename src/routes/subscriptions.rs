use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::AppBaseUrl;
use actix_web::web::Form;
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub name: String,
    pub email: String,
}

impl TryFrom<SubscribeFormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubscribeFormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name  = "Adding new susbcriber",
    skip(form, pool, email_client, app_base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name  = %form.name
    )
)]
#[post("/subscribe")]
pub async fn subscribe(
    form: Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    app_base_url: web::Data<AppBaseUrl>
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&pool, &new_subscriber).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(&email_client, new_subscriber, &app_base_url.0)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(name = "Saving new subscriber into the database", skip(pool))]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES($1, $2, $3, $4, 'pending_confirmation')
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

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &String
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!("{}/subscriptions/confirm?subscription_token=mytoken", base_url);
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />\
        Click <a href=\"{}\">Here</a> to confirm your subscription",
        confirmation_link
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}
