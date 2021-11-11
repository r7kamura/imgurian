use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_client_id: String,
    user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let client = Client::builder().client_id(opt.imgur_client_id).build()?;
    let account = client.get_account(opt.user_name).send().await?;
    dbg!(account);
    Ok(())
}
