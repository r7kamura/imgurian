use crate::client::Client;
use crate::models::BasicWithBoolData;
use crate::result::Result;

#[derive(serde::Serialize)]
pub struct UpdateImage<'a> {
    #[serde(skip)]
    client: &'a Client,

    #[serde(skip)]
    image_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl<'a> UpdateImage<'a> {
    pub fn new(client: &'a Client, image_hash: String) -> Self {
        Self {
            client,
            image_hash,
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

    pub async fn send(self) -> Result<BasicWithBoolData> {
        self.client
            .post(format!("/3/image/{}", self.image_hash), Some(&self))
            .await
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
        let image_hash = "1234567890abcdef";
        Mock::given(method("POST"))
            .and(path(format!("/3/image/{}", image_hash)))
            .respond_with(ResponseTemplate::new(200).set_body_string(include_str!(
                "../../tests/fixtures/basic_with_bool_data.json"
            )))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client.update_image(image_hash).send().await;
        assert!(result.is_ok());
    }
}
