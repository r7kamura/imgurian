use crate::client::Client;
use crate::models::Images;
use crate::result::Result;

pub struct ListAccountImages<'a> {
    client: &'a Client,
    user_name: String,
}

impl<'a> ListAccountImages<'a> {
    pub fn new(client: &'a Client, user_name: String) -> Self {
        Self { client, user_name }
    }

    pub async fn send(self) -> Result<Images> {
        self.client
            .get(format!("/3/account/{}/images", self.user_name))
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
