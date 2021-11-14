use crate::client::Client;
use crate::opt::UpdateImageInput;
use crate::result::Result;

pub async fn update_image(input: UpdateImageInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let mut builder = client.update_image(input.hash);
    if let Some(value) = input.description {
        builder = builder.description(value);
    }
    if let Some(value) = input.title {
        builder = builder.title(value);
    }
    let model = builder.send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
