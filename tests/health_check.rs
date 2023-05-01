#[tokio::test]
async fn health_check_test() {
    spawn_app().await.expect("Failed to spawn app");

    let client = reqwest::Client::new();
    let uri = "localhost:8080/health_check";
    let response = client
        .get(uri)
        .send()
        .await 
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    newsletter_rs::run().await
}
