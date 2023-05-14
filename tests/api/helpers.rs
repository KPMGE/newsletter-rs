use newsletter_rs::configuration::{get_configuration, DbSettings};
use newsletter_rs::email_client::EmailClient;
use newsletter_rs::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::time::Duration;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // we  bind to 0, so we get a random port assigned by the OS for us
    let listener = TcpListener::bind("localhost:0").expect("Coult not start tcp listener");

    // port assigned by OS
    let port = listener.local_addr().unwrap().port();
    let mut configs = get_configuration().expect("could not read configuration file!");

    // get a random database name
    let random_db_name = Uuid::new_v4()
        .to_string()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    configs.database.db_name = format!("db_{}", random_db_name);

    let pool = configure_database(&configs.database).await;
    let sender = configs.email_client.sender().expect("Invalid sender email address");
    let base_url = configs.email_client.base_url;
    let authorization_token = Secret::new("test-token".to_string());
    let email_client = EmailClient::new(base_url, sender, authorization_token, Duration::from_millis(100));
    let server =
        newsletter_rs::startup::run(listener, pool.clone(), email_client).expect("Could not start server");
    let address = format!("http://localhost:{}", port);
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: pool,
    }
}

async fn configure_database(config: &DbSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.get_connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE {};"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.get_connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
