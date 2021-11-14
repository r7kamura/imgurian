#[derive(Debug)]
pub enum Error {
    ImgurError {
        details: crate::models::Error,
    },
    JsonDeserializeError {
        source: serde_path_to_error::Error<serde_json::Error>,
    },
    JsonSerializeError {
        source: serde_json::Error,
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

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::JsonSerializeError { source }
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for Error {
    fn from(source: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::JsonDeserializeError { source }
    }
}
