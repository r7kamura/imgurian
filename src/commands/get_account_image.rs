use crate::opt::GetAccountImageInput;
use crate::result::Result;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account_image(input: GetAccountImageInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model =
        account_api::get_account_image(&configuration, &input.user_name, &input.image_id).await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
