use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    access_token: String,
    account_id: u32,
    account_username: String,
    expires_in: u32,
    refresh_token: String,
    scope: Option<String>,
    token_type: String,
}
