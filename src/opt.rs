use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Imgur API client.")]
pub enum Opt {
    #[structopt(about = "Delete an image.")]
    DeleteImage { client_id: String, hash: String },

    #[structopt(about = "Favorite an image.")]
    FavoriteImage { client_id: String, hash: String },

    #[structopt(about = "Generates an access token from given refresh token.")]
    GenerateAccessToken {
        client_id: String,
        client_secret: String,
        refresh_token: String,
    },

    #[structopt(about = "Get information about an account.")]
    GetAccount {
        client_id: String,
        user_name: String,
    },

    #[structopt(about = "Get information about an image.")]
    GetImage { client_id: String, hash: String },

    #[structopt(about = "List account images.")]
    ListAccountImages {
        access_token: String,
        user_name: String,
        #[structopt(long)]
        page: Option<u32>,
        #[structopt(long)]
        per_page: Option<u8>,
    },

    #[structopt(about = "Update information about an image.")]
    UpdateImage {
        client_id: String,
        hash: String,
        #[structopt(long)]
        description: Option<String>,
        #[structopt(long)]
        title: Option<String>,
    },

    #[structopt(about = "Upload a new image.")]
    UploadImage {
        client_id: String,
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
