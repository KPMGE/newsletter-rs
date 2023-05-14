use newsletter_rs::email_client::EmailClient;
use newsletter_rs::telemetry::{get_subscriber, init_subscriber};
use newsletter_rs::{configuration, startup::run};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter_rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configs = configuration::get_configuration().expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();
    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy(&db_connection_string.expose_secret())
        .expect("could not connect to the database");

    let timeout = configs.email_client.timeout();
    let sender_email = configs.email_client.sender().expect("Invalid sender email address");
    let base_url = configs.email_client.base_url;
    let authorization_token = configs.email_client.authorization_token;
    let email_client = EmailClient::new(base_url, sender_email, authorization_token, timeout);

    let address = format!("{}:{}", configs.app.host, configs.app.port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");

    run(listener, pool, email_client)?.await
}
