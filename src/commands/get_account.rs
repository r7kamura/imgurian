use crate::client::Client;
use crate::opt::GetAccountInput;
use crate::result::Result;

pub async fn get_account(input: GetAccountInput) -> Result<()> {
    let client = Client::builder()
        .credentials(input.access_token, input.client_id)
        .build()?;
    let model = client.get_account(input.user_name).send().await?;
    let json = serde_json::to_string(&model)?;
    println!("{}", json);
    Ok(())
}
