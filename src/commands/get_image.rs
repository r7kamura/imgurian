use crate::client::Client;
use crate::opt::GetImageInput;
use crate::result::Result;

pub async fn get_image(input: GetImageInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let model = client.get_image(input.hash).send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
