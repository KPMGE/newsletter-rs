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


    let address = format!("{}:{}", configs.app.host, configs.app.port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");

    run(listener, pool)?.await
}
