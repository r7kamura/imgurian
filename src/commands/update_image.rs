use crate::client::Client;
use crate::result::Result;

pub async fn update_image(
    client_id: String,
    hash: String,
    description: Option<String>,
    title: Option<String>,
) -> Result<()> {
    let client = Client::builder().client_id(client_id).build()?;
    let mut builder = client.update_image(hash);
    if let Some(value) = description {
        builder = builder.description(value);
    }
    if let Some(value) = title {
        builder = builder.title(value);
    }
    let image = builder.send().await?;
    dbg!(image);
    Ok(())
}
