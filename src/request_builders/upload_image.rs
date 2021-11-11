use crate::client::Client;
use crate::models::Image;
use crate::result::Result;
use base64;

#[derive(serde::Serialize)]
pub struct UploadImage<'a> {
    #[serde(skip)]
    client: &'a Client,

    image: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl<'a> UploadImage<'a> {
    pub fn new(client: &'a Client, image: Vec<u8>) -> Self {
        Self {
            client,
            image: base64::encode(image),
            description: None,
            title: None,
        }
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub async fn send(self) -> Result<Image> {
        let path = "/3/image".to_string();
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
        Mock::given(method("POST"))
            .and(path("/3/image"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../../tests/fixtures/image.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let image = std::fs::read("tests/fixtures/image.png").unwrap();
        let result = client.upload_image(image).send().await;
        assert!(result.is_ok());
    }
}
