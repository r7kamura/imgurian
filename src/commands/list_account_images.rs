use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn list_account_images(opt: Opt) -> Result<()> {
    if let Opt::ListAccountImages {
        access_token,
        page,
        per_page,
        user_name,
    } = opt
    {
        let client = Client::builder().access_token(access_token).build()?;
        let mut builder = client.list_account_images(user_name);
        if let Some(value) = page {
            builder = builder.page(value);
        }
        if let Some(value) = per_page {
            builder = builder.per_page(value);
        }
        let model = builder.send().await?;
        let json = serde_json::to_string(&model)?;
        println!("{}", json);
        Ok(())
    } else {
        panic!()
    }
}
