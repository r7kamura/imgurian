use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn get_image(opt: Opt) -> Result<()> {
    if let Opt::GetImage {
        access_token,
        client_id,
        hash,
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
        let basic = client.get_image(hash).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
