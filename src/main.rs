use imgurian::commands::{
    delete_image, favorite_image, generate_access_token, get_account, get_image,
    list_account_images, update_image, upload_image,
};
use imgurian::opt::Opt;
use imgurian::result::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::DeleteImage { client_id, hash } => delete_image(client_id, hash).await?,
        Opt::FavoriteImage { client_id, hash } => favorite_image(client_id, hash).await?,
        Opt::GenerateAccessToken {
            client_id,
            client_secret,
            refresh_token,
        } => generate_access_token(client_id, client_secret, refresh_token).await?,
        Opt::GetAccount {
            client_id,
            user_name,
        } => get_account(client_id, user_name).await?,
        Opt::GetImage { client_id, hash } => get_image(client_id, hash).await?,
        Opt::ListAccountImages {
            access_token,
            user_name,
            page,
            per_page,
        } => list_account_images(access_token, user_name, page, per_page).await?,
        Opt::UpdateImage {
            client_id,
            hash,
            description,
            title,
        } => update_image(client_id, hash, description, title).await?,
        Opt::UploadImage {
            album,
            client_id,
            description,
            file_path,
            name,
            title,
        } => upload_image(album, client_id, description, file_path, name, title).await?,
    }
    Ok(())
}
