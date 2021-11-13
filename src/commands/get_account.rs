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
        let mut client_builder = Client::builder();
        if let Some(value) = access_token {
            client_builder = client_builder.access_token(value)
        }
        if let Some(value) = client_id {
            client_builder = client_builder.client_id(value)
        }
        let client = client_builder.build()?;
        let basic = client.get_account(user_name).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
