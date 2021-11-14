use crate::client::Client;
use crate::opt::DeleteImageInput;
use crate::result::Result;

pub async fn delete_image(input: DeleteImageInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let model = client.delete_image(input.hash).send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
