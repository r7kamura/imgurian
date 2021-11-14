use crate::client::Client;
use crate::opt::UploadImageInput;
use crate::result::Result;
use std::fs;

pub async fn upload_image(opt: UploadImageInput) -> Result<()> {
    let client = Client::builder()
        .credentials(opt.access_token, opt.client_id)
        .build()?;
    let bytes = fs::read(opt.file_path).unwrap();
    let mut builder = client.upload_image(bytes);
    if let Some(value) = opt.album {
        builder = builder.album(value);
    }
    if let Some(value) = opt.description {
        builder = builder.description(value);
    }
    if let Some(value) = opt.name {
        builder = builder.name(value);
    }
    if let Some(value) = opt.title {
        builder = builder.title(value);
    }
    let model = builder.send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
