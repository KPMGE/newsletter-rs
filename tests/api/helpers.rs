use newsletter_rs::configuration::{get_configuration, DbSettings};
use newsletter_rs::startup::{get_connection_pool, Application};
use newsletter_rs::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

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
    pub email_server: MockServer,
    pub port: u16
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/subscribe", self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let mock_email_server = MockServer::start().await;

    // randomise configuration to ensure code isolation
    let configuration = {
        // get a random database name
        let random_db_name = Uuid::new_v4()
            .to_string()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        let mut conf = get_configuration().expect("failed to read configuration file");
        // different database for each test
        conf.database.db_name = format!("db_{}", random_db_name);
        // use random OS-given port
        conf.app.port = 0;
        // use mock email server as the api
        conf.email_client.base_url = mock_email_server.uri();
        conf
    };

    let pool = get_connection_pool(&configuration.database);

    // create and migrate database
    configure_database(&configuration.database).await;

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let address = format!("http://localhost:{}", application.port());
    let port = application.port().clone();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: pool,
        email_server: mock_email_server,
        port 
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
