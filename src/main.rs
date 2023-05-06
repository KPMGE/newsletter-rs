use newsletter_rs::telemetry::{get_subscriber, init_subscriber};
use newsletter_rs::{configuration, startup::run};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter_rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configs = configuration::get_configuration().expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();
    let pool = PgPool::connect(&db_connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("{}:{}", configs.app.host, configs.app.port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");

    run(listener, pool)?.await
}
