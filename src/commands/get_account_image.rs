use crate::client::Client;
use crate::opt::GetAccountImageInput;
use crate::result::Result;

pub async fn get_account_image(input: GetAccountImageInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let model = client
        .get_account_image(input.user_name, input.image_id)
        .send()
        .await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
