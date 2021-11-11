use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_client_id: String,
    delete_hash: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let client = Client::builder().client_id(opt.imgur_client_id).build()?;
    let basic = client.delete_image(opt.delete_hash).send().await?;
    dbg!(basic);
    Ok(())
}
