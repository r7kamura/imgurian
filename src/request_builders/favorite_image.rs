use crate::client::Client;
use crate::models::BasicWithStringData;
use crate::Result;

pub struct FavoriteImage<'a> {
    client: &'a Client,
    image_hash: String,
}

impl<'a> FavoriteImage<'a> {
    pub fn new(client: &'a Client, image_hash: String) -> Self {
        Self { client, image_hash }
    }

    pub async fn send(self) -> Result<BasicWithStringData> {
        self.client
            .post(
                format!("/3/image/{}/favorite", self.image_hash),
                None::<&()>,
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
        let image_hash = "1234567890abcdef";
        Mock::given(method("POST"))
            .and(path(format!("/3/image/{}/favorite", image_hash)))
            .respond_with(ResponseTemplate::new(200).set_body_string(include_str!(
                "../../tests/fixtures/basic_with_string_data.json"
            )))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client.favorite_image(image_hash).send().await;
        assert!(result.is_ok());
    }
}
