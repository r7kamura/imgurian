use crate::commands::map_to_api_key;
use crate::opt::UpdateImageInput;
use imgur_openapi::apis::configuration::Configuration;
use imgur_openapi::apis::image_api;

pub async fn update_image(input: UpdateImageInput) {
    let mut configuration = Configuration::new();
    configuration.api_key = map_to_api_key(input.client_id);
    configuration.oauth_access_token = input.access_token;
    let model = image_api::update_image(
        &configuration,
        &input.hash,
        input.description.as_deref(),
        input.title.as_deref(),
    )
    .await
    .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
