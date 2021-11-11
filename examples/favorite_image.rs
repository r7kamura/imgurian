use imguria::client::Client;
use imguria::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let access_token =
        env::var("IMGUR_ACCESS_TOKEN").expect("Specify IMGUR_ACCESS_TOKEN environment variable.");
    let client = Client::builder().access_token(access_token).build()?;
    let basic = client.favorite_image("Oxk2SNG").send().await?;
    dbg!(basic);
    Ok(())
}
