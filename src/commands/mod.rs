use imgur_openapi::apis::configuration::ApiKey;

mod delete_image;
mod favorite_image;
mod generate_access_token;
mod get_account;
mod get_account_image;
mod get_account_images_count;
mod get_image;
mod list_account_images;
mod update_image;
mod upload_image;

pub use delete_image::*;
pub use favorite_image::*;
pub use generate_access_token::*;
pub use get_account::*;
pub use get_account_image::*;
pub use get_account_images_count::*;
pub use get_image::*;
pub use list_account_images::*;
pub use update_image::*;
pub use upload_image::*;

// Convert optional client_id into imgur_api compatible format.
fn map_to_api_key(client_id: Option<String>) -> Option<ApiKey> {
    client_id.map(|value| ApiKey {
        prefix: None,
        key: format!("Client-ID {value}", value = value),
    })
}
