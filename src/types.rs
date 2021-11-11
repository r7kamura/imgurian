// This module defines some useful types for this crate.

use crate::models;

#[derive(Debug)]
pub enum Error {
    ImgurError {
        details: models::Error,
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

pub type Result<T, E = Error> = std::result::Result<T, E>;
