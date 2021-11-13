use crate::client::Client;
use crate::result::Result;
use std::fs;

pub async fn upload_image(
    album: Option<String>,
    client_id: String,
    description: Option<String>,
    file_path: String,
    name: Option<String>,
    title: Option<String>,
) -> Result<()> {
    let bytes = fs::read(file_path).unwrap();
    let client = Client::builder().client_id(client_id).build()?;
    let mut builder = client.upload_image(bytes);
    if let Some(value) = album {
        builder = builder.album(value);
    }
    if let Some(value) = description {
        builder = builder.description(value);
    }
    if let Some(value) = name {
        builder = builder.name(value);
    }
    if let Some(value) = title {
        builder = builder.title(value);
    }
    let image = builder.send().await?;
    dbg!(image);
    Ok(())
}
