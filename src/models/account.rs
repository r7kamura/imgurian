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
