use imgurian::commands::{
    delete_image, favorite_image, generate_access_token, get_account, get_account_image,
    get_account_images_count, get_image, list_account_images, update_image, upload_image,
};
use imgurian::opt::Opt;
use imgurian::result::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::DeleteImage(input) => delete_image(input).await?,
        Opt::FavoriteImage(input) => favorite_image(input).await?,
        Opt::GenerateAccessToken(input) => generate_access_token(input).await?,
        Opt::GetAccount(input) => get_account(input).await?,
        Opt::GetAccountImage(input) => get_account_image(input).await?,
        Opt::GetAccountImagesCount(input) => get_account_images_count(input).await?,
        Opt::GetImage(input) => get_image(input).await?,
        Opt::ListAccountImages(input) => list_account_images(input).await?,
        Opt::UpdateImage(input) => update_image(input).await?,
        Opt::UploadImage(input) => upload_image(input).await?,
    }
    Ok(())
}
