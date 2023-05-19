use super::helpers::spawn_app;
use serde_json::Value;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=testname", "missing name"),
        ("name=testmail", "missing email"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_string()).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Assertion error with payload: {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com".to_string();

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_new_subscriber() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com".to_string();

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body).await;

    let data_saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!(data_saved.name, "le guin");
    assert_eq!(data_saved.status, "pending_confirmation");
    assert_eq!(data_saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn suscribe_returns_400_when_data_is_present_but_invalid() {
    let app = spawn_app().await;
    let body = "name=()<>=ursula_le_guin%40gmail.com".to_string();
    let response = app.post_subscriptions(body).await;
    assert_eq!(400, response.status());
}

#[tokio::test]
async fn subscribe_sends_confirmation_email_with_link_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com".to_string();

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body).await;

    let email_request = &app.email_server.received_requests().await.unwrap();

    let body: Value = serde_json::from_slice(&email_request[0].body).unwrap();

    // extract body from one of the fields
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);

        links[0].as_str().to_owned()
    };

    let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(&body["TextBody"].as_str().unwrap());

    assert_eq!(html_link, text_link);
}
