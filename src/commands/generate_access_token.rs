use crate::client::Client;
use crate::result::Result;

pub async fn generate_access_token(
    client_id: String,
    client_secret: String,
    refresh_token: String,
) -> Result<()> {
    let basic = Client::builder()
        .build()?
        .generate_access_token(client_id, client_secret, refresh_token)
        .send()
        .await?;
    dbg!(basic);
    Ok(())
}
