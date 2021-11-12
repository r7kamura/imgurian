use crate::error::Error::ImgurError;
use crate::request_builders::{
    DeleteImage, FavoriteImage, GetAccount, GetImage, ListAccountImages, UpdateImage, UploadImage,
};
use crate::result::Result;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn delete_image(&self, image_hash: impl Into<String>) -> DeleteImage {
        DeleteImage::new(self, image_hash.into())
    }

    pub fn get_account(&self, user_name: impl Into<String>) -> GetAccount {
        GetAccount::new(self, user_name.into())
    }

    pub fn get_image(&self, image_hash: impl Into<String>) -> GetImage {
        GetImage::new(self, image_hash.into())
    }

    pub fn favorite_image(&self, image_hash: impl Into<String>) -> FavoriteImage {
        FavoriteImage::new(self, image_hash.into())
    }

    pub fn list_account_images(&self, user_name: impl Into<String>) -> ListAccountImages {
        ListAccountImages::new(self, user_name.into())
    }

    pub fn update_image(&self, image_hash: impl Into<String>) -> UpdateImage {
        UpdateImage::new(self, image_hash.into())
    }

    pub fn upload_image(&self, image: Vec<u8>) -> UploadImage {
        UploadImage::new(self, image)
    }

    pub async fn delete<T, A>(&self, path: A) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        A: AsRef<str>,
    {
        let response = self
            .client
            .delete(format!("{}{}", self.base_url, path.as_ref()))
            .send()
            .await?;
        let response = map_unsuccess_to_imgur_error(response).await?;
        let model = map_json_to_model(response).await?;
        Ok(model)
    }

    pub async fn get<T, A>(&self, path: A) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        A: AsRef<str>,
    {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path.as_ref()))
            .send()
            .await?;
        let response = map_unsuccess_to_imgur_error(response).await?;
        let model = map_json_to_model(response).await?;
        Ok(model)
    }

    pub async fn post<T, A, S>(&self, path: A, parameters: Option<S>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        A: AsRef<str>,
        S: serde::Serialize,
    {
        let response = self
            .client
            .post(format!("{}{}", self.base_url, path.as_ref()))
            .form(&parameters)
            .send()
            .await?;
        let response = map_unsuccess_to_imgur_error(response).await?;
        let model = map_json_to_model(response).await?;
        Ok(model)
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
            .user_agent("imgurian")
            .default_headers(map)
            .build()?;
        Ok(Client {
            base_url: self.base_url,
            client,
        })
    }

    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into();
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
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

async fn map_json_to_model<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T> {
    let text = response.text().await?;
    let deserializer = &mut serde_json::Deserializer::from_str(&text);
    let model = serde_path_to_error::deserialize(deserializer)?;
    Ok(model)
}

async fn map_unsuccess_to_imgur_error(response: reqwest::Response) -> Result<reqwest::Response> {
    if response.status().is_success() {
        Ok(response)
    } else {
        let details = map_json_to_model(response).await?;
        Err(ImgurError { details })
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
