use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicWithBoolData {
    data: bool,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicWithU32Data {
    data: u32,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicWithStringData {
    data: String,
    success: bool,
    status: u32,
}
