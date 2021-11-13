use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn generate_access_token(opt: Opt) -> Result<()> {
    if let Opt::GenerateAccessToken {
        client_id,
        client_secret,
        refresh_token,
    } = opt
    {
        let basic = Client::builder()
            .build()?
            .generate_access_token(client_id, client_secret, refresh_token)
            .send()
            .await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
