use reqwest::Error;
use std::fmt;
use std::error::Error as StdError;

pub type HttpResult<T> = Result<T, IotCoreError>;

#[derive(Debug)]
pub enum IotCoreError {
    DeviceNotFound,
    HttpError(String),
    OtherError(String),
}

impl From<Error> for IotCoreError {
    fn from(error: Error) -> Self {
        IotCoreError::HttpError(error.to_string())
    }
}

impl From<std::io::Error> for IotCoreError {
    fn from(error: std::io::Error) -> Self {
        IotCoreError::HttpError(error.to_string())
    }
}

impl From<std::num::ParseIntError> for IotCoreError {
    fn from(error: std::num::ParseIntError) -> Self {
        IotCoreError::HttpError(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for IotCoreError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        IotCoreError::HttpError(error.to_string())
    }
}

impl From<serde_json::Error> for IotCoreError {
    fn from(error: serde_json::Error) -> Self {
        IotCoreError::HttpError(error.to_string())
    }
}

impl fmt::Display for IotCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IotCoreError::DeviceNotFound => write!(f, "Device not found"),
            IotCoreError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            IotCoreError::OtherError(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl StdError for IotCoreError {
    fn description(&self) -> &str {
        match *self {
            IotCoreError::DeviceNotFound => "Device not found",
            IotCoreError::HttpError(ref msg) => msg,
            IotCoreError::OtherError(ref err) => err,
        }
    }
}
