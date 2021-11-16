# imgurian

[![test](https://github.com/r7kamura/imgurian/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/imgurian/actions/workflows/test.yml)

[Imgur API](https://apidocs.imgur.com/) client.

## Usage

Install an executable binary from [Releases](https://github.com/r7kamura/imgurian/releases), or run `cargo install imgurian`.

```console
$ imgurian --help
imgurian 0.4.0
Imgur API client.

USAGE:
    imgurian <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete-image                Delete an image.
    favorite-image              Favorite an image.
    generate-access-token       Generates an access token from given refresh token.
    get-account                 Get information about an account.
    get-account-image           Get information about an image of an account.
    get-account-images-count    Get the total number of images associated with the account.
    get-image                   Get information about an image.
    help                        Prints this message or the help of the given subcommand(s)
    list-account-images         List account images.
    update-image                Update information about an image.
    upload-image                Upload a new image.
```
