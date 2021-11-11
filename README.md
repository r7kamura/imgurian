# imguria

[Tokio](https://github.com/tokio-rs/tokio)-based [Imgur API](https://apidocs.imgur.com/) client for Rust.

## Usage

### Build client

```rust
let client_id = std::env::var("IMGUR_CLIENT_ID").unwrap();
let client = imguria::client::Client::builder().client_id(client_id).build()?;
```

### Get Account

```rust
let account = client.get_account("ghostinspector").send().await?;
```

### Get Image

```rust
let image = client.get_image("orunSTu").send().await?;
```

### Upload Image

```rust
let bytes = fs::read(file_path).unwrap();
let image = client.upload_image(bytes).send().await?;
```

### Others

See [examples](/examples) for more examples!
