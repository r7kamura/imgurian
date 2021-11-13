use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;
use std::fs;

pub async fn upload_image(opt: Opt) -> Result<()> {
    if let Opt::UploadImage {
        access_token,
        album,
        client_id,
        description,
        file_path,
        name,
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
        let bytes = fs::read(file_path).unwrap();
        let mut builder = client.upload_image(bytes);
        if let Some(value) = album {
            builder = builder.album(value);
        }
        if let Some(value) = description {
            builder = builder.description(value);
        }
        if let Some(value) = name {
            builder = builder.name(value);
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
