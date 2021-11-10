# imguria

[Imgur API](https://apidocs.imgur.com/) client for Rust.

## Usage

### Build client

```rust
let client_id = std::env::var("IMGUR_CLIENT_ID").unwrap();
let client = imguria::Client::builder().client_id(client_id).build()?;
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
let image = client.upload_image(base64_encoded_image_data).send().await?;
```
