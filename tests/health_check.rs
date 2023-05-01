use std::net::TcpListener;

fn spawn_app() -> String {
    // we  bind to 0, so we get a random port assigned by the OS for us
    let listener = TcpListener::bind("localhost:0").expect("Coult not start tcp listener");

    // port assigned by OS
    let port = listener.local_addr().unwrap().port();

    let server = newsletter_rs::run(listener).expect("Could not start server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await 
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=testname", "missing name"),
        ("name=testmail", "missing email"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscribe", address))
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
