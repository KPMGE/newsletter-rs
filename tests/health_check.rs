#[tokio::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();
    let uri = "http://localhost:8080/health_check";
    let response = client
        .get(uri)
        .send()
        .await 
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = newsletter_rs::run().expect("Could not start server");
    let _ = tokio::spawn(server);
}
