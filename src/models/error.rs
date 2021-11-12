use serde::Deserialize;

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
