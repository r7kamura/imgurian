use crate::commands::map_to_api_key;
use crate::opt::UploadImageInput;
use crate::result::Result;
use base64;
use imgur_openapi::apis::configuration::Configuration;
use imgur_openapi::apis::image_api;
use std::fs;

pub async fn upload_image(input: UploadImageInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.api_key = map_to_api_key(input.client_id);
    configuration.oauth_access_token = input.access_token;
    let bytes = fs::read(input.file_path)?;
    let image = base64::encode(bytes);
    let model = image_api::upload_image(
        &configuration,
        &image,
        input.album.as_deref(),
        input.description.as_deref(),
        input.name.as_deref(),
        input.title.as_deref(),
    )
    .await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
