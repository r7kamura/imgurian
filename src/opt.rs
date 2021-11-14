use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Imgur API client.")]
pub enum Opt {
    #[structopt(about = "Delete an image.")]
    DeleteImage(DeleteImageInput),
    #[structopt(about = "Favorite an image.")]
    FavoriteImage(FavoriteImageInput),
    #[structopt(about = "Generates an access token from given refresh token.")]
    GenerateAccessToken(GenerateAccessTokenInput),
    #[structopt(about = "Get information about an account.")]
    GetAccount(GetAccountInput),
    #[structopt(about = "Get information about an image of an account.")]
    GetAccountImage(GetAccountImageInput),
    #[structopt(about = "Get the total number of images associated with the account.")]
    GetAccountImagesCount(GetAccountImagesCountInput),
    #[structopt(about = "Get information about an image.")]
    GetImage(GetImageInput),
    #[structopt(about = "List account images.")]
    ListAccountImages(ListAccountImagesInput),
    #[structopt(about = "Update information about an image.")]
    UpdateImage(UpdateImageInput),
    #[structopt(about = "Upload a new image.")]
    UploadImage(UploadImageInput),
}

#[derive(Debug, StructOpt)]
pub struct DeleteImageInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub hash: String,
}

#[derive(Debug, StructOpt)]
pub struct FavoriteImageInput {
    #[structopt(long)]
    pub access_token: String,
    pub hash: String,
}

#[derive(Debug, StructOpt)]
pub struct GenerateAccessTokenInput {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Debug, StructOpt)]
pub struct GetAccountInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub user_name: String,
}

#[derive(Debug, StructOpt)]
pub struct GetAccountImageInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub user_name: String,
    pub image_id: String,
}

#[derive(Debug, StructOpt)]
pub struct GetAccountImagesCountInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub user_name: String,
}

#[derive(Debug, StructOpt)]
pub struct GetImageInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub hash: String,
}

#[derive(Debug, StructOpt)]
pub struct ListAccountImagesInput {
    #[structopt(long)]
    pub access_token: String,
    pub user_name: String,
    #[structopt(long)]
    pub page: Option<u32>,
    #[structopt(long)]
    pub per_page: Option<u8>,
}

#[derive(Debug, StructOpt)]
pub struct UpdateImageInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub hash: String,
    #[structopt(long)]
    pub description: Option<String>,
    #[structopt(long)]
    pub title: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct UploadImageInput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub file_path: String,
    #[structopt(long)]
    pub album: Option<String>,
    #[structopt(long)]
    pub description: Option<String>,
    #[structopt(long)]
    pub name: Option<String>,
    #[structopt(long)]
    pub title: Option<String>,
}
