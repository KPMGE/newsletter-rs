use std::net::TcpListener;
use newsletter_rs::{startup::run, configuration};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let configs = configuration::get_configuration()
        .expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();
    let pool = PgPool::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("{}:{}", configs.app_host, configs.app_port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");

    run(listener, pool)?.await
}
