use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let basic = Client::builder()
        .build()?
        .generate_access_token(opt.client_id, opt.client_secret, opt.refresh_token)
        .send()
        .await?;
    dbg!(basic);
    Ok(())
}
