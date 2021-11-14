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
        let model = Client::builder()
            .build()?
            .generate_access_token(client_id, client_secret, refresh_token)
            .send()
            .await?;
        let json = serde_json::to_string(&model)?;
        println!("{}", json);
        Ok(())
    } else {
        panic!()
    }
}
