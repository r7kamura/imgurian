use imguria::client::Client;
use imguria::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id =
        env::var("IMGUR_CLIENT_ID").expect("Specify IMGUR_CLIENT_ID environment variable.");
    let client = Client::builder().client_id(client_id).build()?;
    let image = client.get_image("dpFFLZL").send().await?;
    dbg!(image);
    Ok(())
}
