use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn get_account_images_count(opt: Opt) -> Result<()> {
    if let Opt::GetAccountImagesCount {
        access_token,
        client_id,
        user_name,
    } = opt
    {
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
        let model = client.get_account_images_count(user_name).send().await?;
        let json = serde_json::to_string(&model)?;
        println!("{}", json);
        Ok(())
    } else {
        panic!()
    }
}
