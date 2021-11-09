use reqwest;

pub struct Client {
    access_token: Option<String>,
    base_url: String,
    client_id: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            access_token: None,
            base_url: "https://api.imgur.com".to_string(),
            client_id: None,
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn access_token(mut self, value: String) -> Self {
        self.access_token = Some(value);
        self
    }

    pub fn base_url(mut self, value: String) -> Self {
        self.base_url = value;
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub async fn get_image(
        &self,
        image_hash: impl Into<String>,
    ) -> Result<crate::models::Image, reqwest::Error> {
        let url = format!("{}/3/image/{}", self.base_url, image_hash.into());
        let client = reqwest::Client::new();
        let mut builder = client.get(url);
        if let Some(value) = self.authorization_header_value() {
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }
        builder.send().await?.json().await
    }

    pub async fn upload_image(
        &self,
        image: String,
    ) -> Result<crate::models::Image, reqwest::Error> {
        let url = format!("{}/3/image", self.base_url);
        let client = reqwest::Client::new();
        let mut builder = client.post(url);
        if let Some(value) = self.authorization_header_value() {
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }
        let form = [("image", image)];
        builder.form(&form).send().await?.json().await
    }

    fn authorization_header_value(&self) -> Option<String> {
        if let Some(value) = &self.access_token {
            Some(format!("Bearer {}", value))
        } else if let Some(value) = &self.client_id {
            Some(format!("Client-ID {}", value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use wiremock::{
        matchers::{any, header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn client_authorization_header_with_access_token() {
        let server = MockServer::start().await;
        Mock::given(any())
            .and(header("Authorization", "Bearer dummy_access_token"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new()
            .base_url(server.uri())
            .access_token("dummy_access_token".to_string());
        let _ = client.get_image("1234567890abcdef").await;
    }

    #[tokio::test]
    async fn client_authorization_header_with_client_id() {
        let server = MockServer::start().await;
        Mock::given(any())
            .and(header("Authorization", "Client-ID dummy_client_id"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new()
            .base_url(server.uri())
            .client_id("dummy_client_id".to_string());
        let _ = client.get_image("1234567890abcdef").await;
    }

    #[tokio::test]
    async fn client_authorization_header_with_access_token_and_client_id() {
        let server = MockServer::start().await;
        Mock::given(any())
            .and(header("Authorization", "Bearer dummy_access_token"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new()
            .base_url(server.uri())
            .access_token("dummy_access_token".to_string())
            .client_id("dummy_client_id".to_string());
        let _ = client.get_image("1234567890abcdef").await;
    }

    #[tokio::test]
    async fn client_get_image_sends_http_request() {
        let server = MockServer::start().await;
        let image_hash = "1234567890abcdef";
        Mock::given(method("GET"))
            .and(path(format!("/3/image/{}", image_hash)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../tests/fixtures/image.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new().base_url(server.uri());
        let result = client.get_image(image_hash).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn client_upload_image_sends_http_request() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/3/image"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../tests/fixtures/image.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new().base_url(server.uri());
        let image = std::fs::read("tests/fixtures/image.png").unwrap();
        let result = client.upload_image(base64::encode(&image)).await;
        assert!(result.is_ok());
    }
}
