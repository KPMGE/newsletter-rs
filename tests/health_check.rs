use std::net::TcpListener;

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

fn spawn_app() -> String {
    // we  bind to 0, so we get a random port assigned by the OS for us
    let listener = TcpListener::bind("localhost:0").expect("Coult not start tcp listener");

    // port assigned by OS
    let port = listener.local_addr().unwrap().port();

    let server = newsletter_rs::run(listener).expect("Could not start server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
