use imgurian::client::Client;
use imgurian::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id =
        env::var("IMGUR_CLIENT_ID").expect("Specify IMGUR_CLIENT_ID environment variable.");
    let client = Client::builder().client_id(client_id).build()?;
    let basic = client
        .update_image("5LRukel6PzSi5H3")
        .title("test2")
        .description("test2")
        .send()
        .await?;
    dbg!(basic);
    Ok(())
}
