use crate::opt::GetAccountInput;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account(input: GetAccountInput) {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = input.access_token;
    let model = account_api::get_account(&configuration, &input.user_name)
        .await
        .unwrap();
    let json = serde_json::to_string(&model).unwrap();
    println!("{}", json);
}
