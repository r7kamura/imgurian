use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    data: AccountData,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    avatar: Option<String>,
    bio: Option<String>,
    created: u32,
    id: u32,
    pro_expiration: bool,
    reputation_name: String,
    reputation: i32,
    url: String,
    user_follow: AccountUserFollow,
}

#[derive(Debug, Deserialize)]
pub struct AccountUserFollow {
    status: bool,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    data: ErrorData,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    error: String,
    method: String,
    request: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    data: ImageData,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct ImageData {
    account_id: Option<u32>,
    account_url: Option<String>,
    ad_type: u32,
    ad_url: String,
    animated: bool,
    bandwidth: u32,
    datetime: u32,
    description: Option<String>,
    favorite: bool,
    height: u32,
    id: String,
    in_gallery: bool,
    in_most_viral: bool,
    is_ad: bool,
    link: String,
    nsfw: Option<bool>,
    section: Option<String>,
    size: u32,
    tags: Vec<String>,
    title: Option<String>,
    r#type: String,
    views: u32,
    vote: Option<String>,
    width: u32,
}
