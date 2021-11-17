use crate::opt::ListAccountImagesInput;
use crate::result::Result;
use imgur_openapi::apis::account_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn list_account_images(input: ListAccountImagesInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.oauth_access_token = Some(input.access_token);
    let model = account_api::get_account_images(&configuration, &input.user_name).await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
