use crate::client::Client;
use crate::opt::GenerateAccessTokenInput;
use crate::result::Result;

pub async fn generate_access_token(input: GenerateAccessTokenInput) -> Result<()> {
    let model = Client::builder()
        .build()?
        .generate_access_token(input.client_id, input.client_secret, input.refresh_token)
        .send()
        .await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
