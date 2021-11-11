# imgurian

[Tokio](https://github.com/tokio-rs/tokio)-based [Imgur API](https://apidocs.imgur.com/) client for Rust.

## Usage

```rust
use imgurian::client::Client;
use std::fs;

#[tokio::main]
async fn main() {
    let bytes = fs::read("/path/to/image").unwrap();
    let client = Client::builder().client_id("YOUR_CLIENT_ID").build().unwrap();
    let image = client.upload_image(bytes).send().await.unwrap();
    dbg!(image);
}
```

See [examples](/examples) for more.
