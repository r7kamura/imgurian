use imgur_openapi::apis;
use std::io;

#[derive(Debug)]
pub enum Error {
    ApiOthers,
    ApiResponse(Response),
    Io(io::Error),
    JsonSerialize(serde_json::Error),
}

#[derive(Debug)]
pub struct Response {
    content: String,
    status: u16,
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error::Io(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::JsonSerialize(source)
    }
}

impl<T> From<apis::Error<T>> for Error {
    fn from(source: apis::Error<T>) -> Self {
        match source {
            apis::Error::ResponseError(apis::ResponseContent {
                status, content, ..
            }) => Error::ApiResponse(Response {
                status: status.as_u16(),
                content,
            }),
            _ => Error::ApiOthers,
        }
    }
}
