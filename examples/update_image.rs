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
    if let Some(title) = opt.title {
        builder = builder.title(title);
    }
    if let Some(description) = opt.description {
        builder = builder.description(description);
    }
    let basic = builder.send().await?;
    dbg!(basic);
    Ok(())
}
