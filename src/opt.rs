use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Imgur API client.")]
pub enum Opt {
    #[structopt(about = "Delete an image.")]
    DeleteImage {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        hash: String,
    },

    #[structopt(about = "Favorite an image.")]
    FavoriteImage {
        #[structopt(long)]
        access_token: String,
        hash: String,
    },

    #[structopt(about = "Generates an access token from given refresh token.")]
    GenerateAccessToken {
        client_id: String,
        client_secret: String,
        refresh_token: String,
    },

    #[structopt(about = "Get information about an account.")]
    GetAccount {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        user_name: String,
    },

    #[structopt(about = "Get information about an image of an account.")]
    GetAccountImage {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        user_name: String,
        image_id: String,
    },

    #[structopt(about = "Get information about an image.")]
    GetImage {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        hash: String,
    },

    #[structopt(about = "List account images.")]
    ListAccountImages {
        #[structopt(long)]
        access_token: String,
        user_name: String,
        #[structopt(long)]
        page: Option<u32>,
        #[structopt(long)]
        per_page: Option<u8>,
    },

    #[structopt(about = "Update information about an image.")]
    UpdateImage {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        hash: String,
        #[structopt(long)]
        description: Option<String>,
        #[structopt(long)]
        title: Option<String>,
    },

    #[structopt(about = "Upload a new image.")]
    UploadImage {
        #[structopt(long)]
        access_token: Option<String>,
        #[structopt(long)]
        client_id: Option<String>,
        file_path: String,
        #[structopt(long)]
        album: Option<String>,
        #[structopt(long)]
        description: Option<String>,
        #[structopt(long)]
        name: Option<String>,
        #[structopt(long)]
        title: Option<String>,
    },
}
