use crate::opt::FavoriteImageInput;
use crate::result::Result;
use imgur_openapi::apis::configuration::Configuration;
use imgur_openapi::apis::image_api;

pub async fn favorite_image(input: FavoriteImageInput) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(input.access_token);
    let model = image_api::favorite_image(&configuration, &input.hash).await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
