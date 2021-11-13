use crate::client::Client;
use crate::result::Result;

pub async fn list_account_images(
    access_token: String,
    user_name: String,
    page: Option<u32>,
    per_page: Option<u8>,
) -> Result<()> {
    let client = Client::builder().access_token(access_token).build()?;
    let mut builder = client.list_account_images(user_name);
    if let Some(value) = page {
        builder = builder.page(value);
    }
    if let Some(value) = per_page {
        builder = builder.per_page(value);
    }
    let basic = builder.send().await?;
    dbg!(basic);
    Ok(())
}
