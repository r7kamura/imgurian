use crate::client::Client;
use crate::models::Image;
use crate::result::Result;

pub struct GetAccountImage<'a> {
    client: &'a Client,
    image_id: String,
    user_name: String,
}

impl<'a> GetAccountImage<'a> {
    pub fn new(client: &'a Client, user_name: String, image_id: String) -> Self {
        Self {
            client,
            image_id,
            user_name,
        }
    }

    pub async fn send(self) -> Result<Image> {
        self.client
            .get(
                format!(
                    "/3/account/{user_name}/image/{image_id}",
                    image_id = self.image_id,
                    user_name = self.user_name,
                ),
                None::<()>,
            )
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
        let image_id = "dummy_image_id";
        Mock::given(method("GET"))
            .and(path(format!(
                "/3/account/{user_name}/image/{image_id}",
                image_id = image_id,
                user_name = user_name,
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../../tests/fixtures/image.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client.get_account_image(user_name, image_id).send().await;
        assert!(result.is_ok());
    }
}
