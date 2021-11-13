use crate::client::Client;
use crate::result::Result;

pub async fn delete_image(client_id: String, hash: String) -> Result<()> {
    let client = Client::builder().client_id(client_id).build()?;
    let basic = client.delete_image(hash).send().await?;
    dbg!(basic);
    Ok(())
}
