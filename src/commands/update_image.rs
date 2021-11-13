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
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
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
