use crate::client::Client;
use crate::opt::GetAccountImagesCountInput;
use crate::result::Result;

pub async fn get_account_images_count(input: GetAccountImagesCountInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let model = client
        .get_account_images_count(input.user_name)
        .send()
        .await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
