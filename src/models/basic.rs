use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasicWithBoolData {
    data: bool,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct BasicWithU32Data {
    data: u32,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct BasicWithStringData {
    data: String,
    success: bool,
    status: u32,
}
