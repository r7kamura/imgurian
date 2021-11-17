use crate::opt::GenerateAccessTokenInput;
use crate::result::Result;
use imgur_openapi::apis::auth_api;
use imgur_openapi::apis::configuration::Configuration;

pub async fn generate_access_token(input: GenerateAccessTokenInput) -> Result<()> {
    let configuration = Configuration::new();
    let model = auth_api::generate_access_token(
        &configuration,
        &input.client_id,
        &input.client_secret,
        "refresh_token",
        &input.refresh_token,
    )
    .await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
