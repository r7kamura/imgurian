use crate::client::Client;
use crate::result::Result;

pub async fn get_account(client_id: String, user_name: String) -> Result<()> {
    let basic = Client::builder()
        .client_id(client_id)
        .build()?
        .get_account(user_name)
        .send()
        .await?;
    dbg!(basic);
    Ok(())
}
