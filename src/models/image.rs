use serde::Deserialize;

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
    deletehash: Option<String>,
    datetime: u32,
    description: Option<String>,
    favorite: bool,
    height: u32,
    id: String,
    in_gallery: bool,
    in_most_viral: bool,
    is_ad: bool,
    link: String,
    name: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct Images {
    data: Vec<ImageData>,
    success: bool,
    status: u32,
}
