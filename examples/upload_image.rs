use imguria::client::Client;
use imguria::error::Error;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id =
        env::var("IMGUR_CLIENT_ID").expect("Specify IMGUR_CLIENT_ID environment variable.");
    let file_path = env::var("IMAGE_PATH").expect("Specify IMAGE_PATH environment variable.");
    let bytes = fs::read(file_path).unwrap();
    let client = Client::builder().client_id(client_id).build()?;
    let image = client.upload_image(bytes).send().await?;
    dbg!(image);
    Ok(())
}
