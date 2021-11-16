#[derive(Debug)]
pub enum Error<T> {
    JsonSerializeError(serde_json::Error),
    ApiError(imgur_openapi::apis::Error<T>),
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(source: serde_json::Error) -> Self {
        Error::JsonSerializeError(source)
    }
}

impl<T> From<imgur_openapi::apis::Error<T>> for Error<T> {
    fn from(source: imgur_openapi::apis::Error<T>) -> Self {
        Error::ApiError(source)
    }
}
