use imgurian::client::Client;
use imgurian::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_client_id: String,
    file_path: String,

    #[structopt(long)]
    album: Option<String>,

    #[structopt(long)]
    description: Option<String>,

    #[structopt(long)]
    name: Option<String>,

    #[structopt(long)]
    title: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let bytes = fs::read(opt.file_path).unwrap();
    let client = Client::builder().client_id(opt.imgur_client_id).build()?;
    let mut builder = client.upload_image(bytes);
    if let Some(value) = opt.album {
        builder = builder.album(value);
    }
    if let Some(value) = opt.description {
        builder = builder.description(value);
    }
    if let Some(value) = opt.name {
        builder = builder.name(value);
    }
    if let Some(value) = opt.title {
        builder = builder.title(value);
    }
    let image = builder.send().await?;
    dbg!(image);
    Ok(())
}
