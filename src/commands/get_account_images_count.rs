use crate::opt::GetAccountImagesCountInput;
use crate::result::Result;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn get_account_images_count(input: GetAccountImagesCountInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model = account_api::get_account_images_count(&configuration, &input.user_name).await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
