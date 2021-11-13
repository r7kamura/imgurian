use crate::client::Client;
use crate::result::Result;

pub async fn get_image(client_id: String, hash: String) -> Result<()> {
    let basic = Client::builder()
        .client_id(client_id)
        .build()?
        .get_image(hash)
        .send()
        .await?;
    dbg!(basic);
    Ok(())
}
