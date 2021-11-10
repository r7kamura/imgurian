use crate::client::Client;
use crate::models::Image;
use crate::Result;

pub struct UploadImage<'a> {
    client: &'a Client,
    image: String,
}

impl<'a> UploadImage<'a> {
    pub fn new(client: &'a Client, image: String) -> Self {
        Self { client, image }
    }

    pub async fn send(self) -> Result<Image> {
        self.client
            .post("/3/image".to_string(), Some([("image", self.image)]))
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
        let result = client.upload_image(base64::encode(&image)).send().await;
        assert!(result.is_ok());
    }
}
