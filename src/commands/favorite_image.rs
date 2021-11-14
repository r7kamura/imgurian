use crate::client::Client;
use crate::opt::FavoriteImageInput;
use crate::result::Result;

pub async fn favorite_image(input: FavoriteImageInput) -> Result<()> {
    let client = Client::builder().access_token(input.access_token).build()?;
    let model = client.favorite_image(input.hash).send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
