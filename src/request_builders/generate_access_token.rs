use crate::client::Client;
use crate::models::AccessToken;
use crate::result::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenerateAccessToken<'a> {
    #[serde(skip)]
    client: &'a Client,

    client_id: String,
    client_secret: String,
    grant_type: String,
    refresh_token: String,
}

impl<'a> GenerateAccessToken<'a> {
    pub fn new(
        client: &'a Client,
        client_id: String,
        client_secret: String,
        refresh_token: String,
    ) -> Self {
        Self {
            client,
            client_id,
            client_secret,
            grant_type: "refresh_token".to_string(),
            refresh_token,
        }
    }

    pub async fn send(self) -> Result<AccessToken> {
        let path = "/oauth2/token";
        self.client.post(path, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn send() {
        let server = MockServer::start().await;
        let client_id = "dummy_client_id";
        let client_secret = "dummy_client_secret";
        let refresh_token = "dummy_refresh_token";
        Mock::given(method("POST"))
            .and(path("/oauth2/token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../../tests/fixtures/access_token.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client
            .generate_access_token(client_id, client_secret, refresh_token)
            .send()
            .await;
        assert!(result.is_ok());
    }
}
