use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn delete_image(opt: Opt) -> Result<()> {
    if let Opt::DeleteImage {
        access_token,
        client_id,
        hash,
    } = opt
    {
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
        let model = client.delete_image(hash).send().await?;
        let json = serde_json::to_string(&model)?;
        println!("{}", json);
        Ok(())
    } else {
        panic!()
    }
}
