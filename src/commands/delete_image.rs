use crate::commands::map_to_api_key;
use crate::opt::DeleteImageInput;
use imgur_openapi::apis::configuration::Configuration;
use imgur_openapi::apis::image_api;

pub async fn delete_image(input: DeleteImageInput) {
    let mut configuration = Configuration::new();
    configuration.api_key = map_to_api_key(input.client_id);
    configuration.bearer_access_token = input.access_token;
    let model = image_api::delete_image(&configuration, &input.hash)
        .await
        .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
