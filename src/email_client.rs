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
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String
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
            from: self.sender.to_string(),
            to: recipient.to_string(),
            subject: subject.to_string(),
            html_body: html_content.to_string(),
            text_body: text_content.to_string()
        };

        self.http_client
            .post(&url)
            .header("X-Postmark-Server-Token", self.authorization_token.expose_secret())
            .json(&request_body)
            .send()
            .await?;

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
    use wiremock::matchers::{header_exists, header, method, path};
    use super::EmailClient;

    use crate::domain::SubscriberEmail;

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client.send_email(subscriber_email, &subject, &content, &content).await;
    }
}
