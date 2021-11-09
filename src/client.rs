pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn get_image(
        &self,
        image_hash: impl Into<String>,
    ) -> Result<crate::models::Image, reqwest::Error> {
        let path = format!("/3/image/{}", image_hash.into());
        self.get(path).await
    }

    pub async fn upload_image(
        &self,
        image: String,
    ) -> Result<crate::models::Image, reqwest::Error> {
        let path = "/3/image".to_string();
        let parameters = [("image", image)];
        self.post(path, Some(parameters)).await
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: String) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        self.client.get(url).send().await?.json().await
    }

    async fn post<T: serde::de::DeserializeOwned, U: serde::Serialize>(
        &self,
        path: String,
        parameters: Option<U>,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .post(url)
            .form(&parameters)
            .send()
            .await?
            .json()
            .await
    }
}

pub struct ClientBuilder {
    access_token: Option<String>,
    base_url: String,
    client_id: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            access_token: None,
            base_url: "https://api.imgur.com".to_string(),
            client_id: None,
        }
    }
}

impl ClientBuilder {
    pub fn build(self) -> Result<Client, reqwest::Error> {
        let mut map = reqwest::header::HeaderMap::new();
        if let Some(value) = self.authorization_header_value() {
            map.append(reqwest::header::AUTHORIZATION, value.parse().unwrap());
        }

        let client = reqwest::Client::builder()
            .user_agent("imguria")
            .default_headers(map)
            .build()?;
        Ok(Client {
            base_url: self.base_url,
            client,
        })
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
        let client = Client::builder()
            .base_url(server.uri())
            .access_token("dummy_access_token".to_string())
            .build()
            .unwrap();
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
        let client = Client::builder()
            .base_url(server.uri())
            .client_id("dummy_client_id".to_string())
            .build()
            .unwrap();
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
        let client = Client::builder()
            .base_url(server.uri())
            .access_token("dummy_access_token".to_string())
            .client_id("dummy_client_id".to_string())
            .build()
            .unwrap();
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
        let client = Client::builder().base_url(server.uri()).build().unwrap();
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
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let image = std::fs::read("tests/fixtures/image.png").unwrap();
        let result = client.upload_image(base64::encode(&image)).await;
        assert!(result.is_ok());
    }
}
