use crate::opt::GetAccountInput;
use crate::result::Result;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account(input: GetAccountInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = input.access_token;
    let model = account_api::get_account(&configuration, &input.user_name).await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
