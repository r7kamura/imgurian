use imgurian::client::Client;
use imgurian::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_client_id: String,
    file_path: String,

    #[structopt(long)]
    title: Option<String>,

    #[structopt(long)]
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let bytes = fs::read(opt.file_path).unwrap();
    let client = Client::builder().client_id(opt.imgur_client_id).build()?;
    let mut builder = client.upload_image(bytes);
    if let Some(title) = opt.title {
        builder = builder.title(title);
    }
    if let Some(description) = opt.description {
        builder = builder.description(description);
    }
    let image = builder.send().await?;
    dbg!(image);
    Ok(())
}
