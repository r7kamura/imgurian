use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_client_id: String,
    delete_hash: String,

    #[structopt(long)]
    title: Option<String>,

    #[structopt(long)]
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let client = Client::builder().client_id(opt.imgur_client_id).build()?;
    let mut builder = client.update_image(opt.delete_hash);
    if let Some(value) = opt.title {
        builder = builder.title(value);
    }
    if let Some(value) = opt.description {
        builder = builder.description(value);
    }
    let basic = builder.send().await?;
    dbg!(basic);
    Ok(())
}
