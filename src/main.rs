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
        Opt::DeleteImage { .. } => delete_image(opt).await?,
        Opt::FavoriteImage { .. } => favorite_image(opt).await?,
        Opt::GenerateAccessToken { .. } => generate_access_token(opt).await?,
        Opt::GetAccount { .. } => get_account(opt).await?,
        Opt::GetAccountImage { .. } => get_account_image(opt).await?,
        Opt::GetAccountImagesCount { .. } => get_account_images_count(opt).await?,
        Opt::GetImage { .. } => get_image(opt).await?,
        Opt::ListAccountImages { .. } => list_account_images(opt).await?,
        Opt::UpdateImage { .. } => update_image(opt).await?,
        Opt::UploadImage { .. } => upload_image(opt).await?,
    }
    Ok(())
}
