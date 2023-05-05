use std::net::TcpListener;
use newsletter_rs::{startup::run, configuration};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use newsletter_rs::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let subscriber = get_subscriber("newsletter_rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configs = configuration::get_configuration()
        .expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();
    let pool = PgPool::connect(&db_connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("{}:{}", configs.app_host, configs.app_port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");

    run(listener, pool)?.await
}
