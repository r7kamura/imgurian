pub mod client;
pub mod models;
pub mod request_builders;

#[derive(Debug)]
pub enum Error {
    ReqwestError { source: reqwest::Error },
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::ReqwestError { source }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
