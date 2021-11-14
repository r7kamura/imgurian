use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    data: ErrorData,
    success: bool,
    status: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorData {
    error: String,
    method: String,
    request: String,
}
