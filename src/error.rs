#[derive(Debug)]
pub enum Error {
    ImgurError {
        details: crate::models::Error,
    },
    JsonError {
        source: serde_path_to_error::Error<serde_json::Error>,
    },
    ReqwestError {
        source: reqwest::Error,
    },
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::ReqwestError { source }
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for Error {
    fn from(source: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::JsonError { source }
    }
}
