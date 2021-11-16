use crate::opt::GetAccountImageInput;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account_image(input: GetAccountImageInput) {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model = account_api::get_account_image(&configuration, &input.user_name, &input.image_id)
        .await
        .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
