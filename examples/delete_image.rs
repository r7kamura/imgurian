use imguria::client::Client;
use imguria::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id =
        env::var("IMGUR_CLIENT_ID").expect("Specify IMGUR_CLIENT_ID environment variable.");
    let client = Client::builder().client_id(client_id).build()?;
    let basic = client.delete_image("kR1KrSxNfQFic2I").send().await?;
    dbg!(basic);
    Ok(())
}
