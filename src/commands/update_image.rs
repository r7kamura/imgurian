use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn update_image(opt: Opt) -> Result<()> {
    if let Opt::UpdateImage {
        access_token,
        client_id,
        description,
        hash,
        title,
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
        let mut builder = client.update_image(hash);
        if let Some(value) = description {
            builder = builder.description(value);
        }
        if let Some(value) = title {
            builder = builder.title(value);
        }
        let image = builder.send().await?;
        dbg!(image);
        Ok(())
    } else {
        panic!()
    }
}
