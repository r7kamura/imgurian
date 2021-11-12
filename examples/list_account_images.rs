use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_access_token: String,
    user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let client = Client::builder()
        .access_token(opt.imgur_access_token)
        .build()?;
    let account = client.list_account_images(opt.user_name).send().await?;
    dbg!(account);
    Ok(())
}
