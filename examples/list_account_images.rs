use imgurian::client::Client;
use imgurian::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    imgur_access_token: String,
    user_name: String,

    #[structopt(long)]
    page: Option<u32>,

    #[structopt(long)]
    per_page: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let client = Client::builder()
        .access_token(opt.imgur_access_token)
        .build()?;
    let mut builder = client.list_account_images(opt.user_name);
    if let Some(value) = opt.page {
        builder = builder.page(value);
    }
    if let Some(value) = opt.per_page {
        builder = builder.per_page(value);
    }
    let account = builder.send().await?;
    dbg!(account);
    Ok(())
}
