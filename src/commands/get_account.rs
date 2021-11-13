use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn get_account(opt: Opt) -> Result<()> {
    if let Opt::GetAccount {
        access_token,
        client_id,
        user_name,
    } = opt
    {
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
        let basic = client.get_account(user_name).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
