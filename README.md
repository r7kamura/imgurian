# imgurian

[![test](https://github.com/r7kamura/imgurian/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/imgurian/actions/workflows/test.yml)

[Imgur API](https://apidocs.imgur.com/) client.

## Usage

### Binary

```console
$ imgurian --help
imgurian 0.2.10
Imgur API client.

USAGE:
    imgurian <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete-image             Delete an image.
    favorite-image           Favorite an image.
    generate-access-token    Generates an access token from given refresh token.
    get-account              Get information about an account.
    get-image                Get information about an image.
    help                     Prints this message or the help of the given subcommand(s)
    list-account-images      List account images.
    update-image             Update information about an image.
    upload-image             Upload a new image.
```

### Library

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
