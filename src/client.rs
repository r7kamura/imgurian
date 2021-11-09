use crate::request_builders::{GetAccount, GetImage, UploadImage};

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn get_account(&self, user_name: impl Into<String>) -> GetAccount {
        GetAccount::new(self, user_name.into())
    }

    pub fn get_image(&self, image_hash: impl Into<String>) -> GetImage {
        GetImage::new(self, image_hash.into())
    }

    pub fn upload_image(&self, image: String) -> UploadImage {
        UploadImage::new(self, image)
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: String,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        self.client.get(url).send().await?.json().await
    }

    pub async fn post<T: serde::de::DeserializeOwned, U: serde::Serialize>(
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
        self.access_token
            .as_ref()
            .map(|value| format!("Bearer {}", value))
            .or_else(|| {
                self.client_id
                    .as_ref()
                    .map(|value| format!("Client-ID {}", value))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use wiremock::{
        matchers::{any, header},
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
        let _ = client.get_image("1234567890abcdef").send().await;
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
        let _ = client.get_image("1234567890abcdef").send().await;
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
        let _ = client.get_image("1234567890abcdef").send().await;
    }
}
