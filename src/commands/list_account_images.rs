use crate::opt::ListAccountImagesInput;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn list_account_images(input: ListAccountImagesInput) {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model = account_api::get_account_images(&configuration, &input.user_name)
        .await
        .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
