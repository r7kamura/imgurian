use crate::client::Client;
use crate::models::Images;
use crate::result::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct ListAccountImages<'a> {
    #[serde(skip)]
    client: &'a Client,

    #[serde(skip)]
    user_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "perPage")]
    per_page: Option<u8>,
}

impl<'a> ListAccountImages<'a> {
    pub fn new(client: &'a Client, user_name: String) -> Self {
        Self {
            client,
            user_name,
            page: None,
            per_page: None,
        }
    }

    pub fn page(mut self, value: impl Into<u32>) -> Self {
        self.page = Some(value.into());
        self
    }

    pub fn per_page(mut self, value: impl Into<u8>) -> Self {
        self.per_page = Some(value.into());
        self
    }

    pub async fn send(self) -> Result<Images> {
        self.client
            .get(format!("/3/account/{}/images", self.user_name), Some(&self))
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
        let user_name = "dummy_user_name";
        Mock::given(method("GET"))
            .and(path(format!("/3/account/{}/images", user_name)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../../tests/fixtures/images.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client.list_account_images(user_name).send().await;
        assert!(result.is_ok());
    }
}
