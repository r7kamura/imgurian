use crate::client::Client;
use crate::opt::ListAccountImagesInput;
use crate::result::Result;

pub async fn list_account_images(input: ListAccountImagesInput) -> Result<()> {
    let client = Client::builder().access_token(input.access_token).build()?;
    let mut builder = client.list_account_images(input.user_name);
    if let Some(value) = input.page {
        builder = builder.page(value);
    }
    if let Some(value) = input.per_page {
        builder = builder.per_page(value);
    }
    let model = builder.send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
