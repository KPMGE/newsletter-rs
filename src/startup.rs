use crate::configuration::{DbSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, publish_newsletter, subscribe};
use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use std::time::Duration;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configs: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&configs.database);
        let timeout = configs.email_client.timeout();
        let sender_email = configs
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let base_url = configs.email_client.base_url;
        let authorization_token = configs.email_client.authorization_token;
        let email_client = EmailClient::new(base_url, sender_email, authorization_token, timeout);

        let address = format!("{}:{}", configs.app.host, configs.app.port);
        let listener = TcpListener::bind(address).expect("could not start tcp listener");
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, pool, email_client, configs.app.base_url)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// we needa define a wrapper type in order to retrieve the url in the
// subscribe handler, cuz context retrieval in actix web is type-based.
pub struct AppBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let app_base_url = Data::new(AppBaseUrl(base_url));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .service(confirm)
            .service(publish_newsletter)
            .app_data(pool.clone())
            .app_data(email_client.clone())
            .app_data(app_base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn get_connection_pool(configs: &DbSettings) -> PgPool {
    let db_connection_string = configs.get_connection_string();
    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy(db_connection_string.expose_secret())
        .expect("could not connect to the database");
    pool
}
