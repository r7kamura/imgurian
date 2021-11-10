use crate::client::Client;
use crate::models::DeleteImage as DeleteImageModel;
use crate::Result;

pub struct DeleteImage<'a> {
    client: &'a Client,
    image_hash: String,
}

impl<'a> DeleteImage<'a> {
    pub fn new(client: &'a Client, image_hash: String) -> Self {
        Self { client, image_hash }
    }

    pub async fn send(self) -> Result<DeleteImageModel> {
        self.client
            .delete(format!("/3/image/{}", self.image_hash))
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
        Mock::given(method("DELETE"))
            .and(path(format!("/3/image/{}", image_hash)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(include_str!("../../tests/fixtures/delete_image.json")),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder().base_url(server.uri()).build().unwrap();
        let result = client.delete_image(image_hash).send().await;
        assert!(result.is_ok());
    }
}