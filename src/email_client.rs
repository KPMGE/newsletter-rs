use reqwest::Client;
use secrecy::{Secret, ExposeSecret};
use serde::Serialize;
use crate::domain::SubscriberEmail;

pub struct EmailClient {
    pub http_client: Client,
    pub base_url: String,
    pub sender: SubscriberEmail,
    pub authorization_token: Secret<String>
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, authorization_token: Secret<String>) -> Self {
        Self {
            http_client: reqwest::Client::new() ,
            base_url,
            sender,
            authorization_token
        }
    }

    pub async fn send_email(
        &self, 
        recipient: SubscriberEmail, 
        subject: &str, 
        html_content: &str, 
        text_content: &str
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject: subject.as_ref(),
            html_body: html_content.as_ref(),
            text_body: text_content.as_ref()
        };

        self.http_client
            .post(&url)
            .header("X-Postmark-Server-Token", self.authorization_token.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::Faker;
    use fake::faker::lorem::en::{Sentence, Paragraph};
    use fake::{faker::internet::en::SafeEmail, Fake};
    use secrecy::Secret;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{header_exists, header, method, path, any};
    use claim::{assert_ok, assert_err};
    use super::EmailClient;
    use crate::domain::SubscriberEmail;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                dbg!(&body);

                return body.get("From").is_some()
                && body.get("To").is_some()
                && body.get("Subject").is_some()
                && body.get("HtmlBody").is_some()
                && body.get("TextBody").is_some();
            }
            false
        }
    }

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let result = email_client.send_email(subscriber_email, &subject, &content, &content).await;
        assert_ok!(result);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));
        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let result = email_client.send_email(subscriber_email, &subject, &content, &content).await;
        assert_err!(result);
    }
}
