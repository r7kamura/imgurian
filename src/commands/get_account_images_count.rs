use crate::opt::GetAccountImagesCountInput;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account_images_count(input: GetAccountImagesCountInput) {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model = account_api::get_account_images_count(&configuration, &input.user_name)
        .await
        .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
