use std::net::TcpListener;

use newsletter_rs::configuration::get_configuration;
use sqlx::{PgConnection, Connection, PgPool};

pub struct TestApp {
    pub address: String, 
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp {
    // we  bind to 0, so we get a random port assigned by the OS for us
    let listener = TcpListener::bind("localhost:0").expect("Coult not start tcp listener");

    // port assigned by OS
    let port = listener.local_addr().unwrap().port();


    let configs = get_configuration().expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();

    let pool = PgPool::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let server = newsletter_rs::startup::run(listener, pool.clone()).expect("Could not start server");
    let address = format!("http://localhost:{}", port);
    let _ = tokio::spawn(server);

    TestApp { 
        address,
        db_pool:  pool
    }
}

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await 
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=testname", "missing name"),
        ("name=testmail", "missing email"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "x-www-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request!");

        assert_eq!(
            response.status().as_u16(), 400,
            "Assertion error with payload: {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let configs = get_configuration().expect("could not read configuration file!");
    let db_connection_string = configs.database.get_connection_string();

    let mut db_connection = PgConnection::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();


    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");


    let data_saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut db_connection)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(data_saved.name, "le guin");
    assert_eq!(data_saved.email, "ursula_le_guin@gmail.com");
}
