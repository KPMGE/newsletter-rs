use super::helpers::spawn_app;

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
    let response = app.post_subscriptions(body).await;

    let data_saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(data_saved.name, "le guin");
    assert_eq!(data_saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn suscribe_returns_400_when_data_is_present_but_invalid() {
    let app = spawn_app().await;
    let body = "name=()<>=ursula_le_guin%40gmail.com".to_string();
    let response = app.post_subscriptions(body).await;
    assert_eq!(400, response.status());
}
