use crate::client::Client;
use crate::result::Result;

pub async fn favorite_image(client_id: String, hash: String) -> Result<()> {
    let client = Client::builder().client_id(client_id).build()?;
    let basic = client.favorite_image(hash).send().await?;
    dbg!(basic);
    Ok(())
}
