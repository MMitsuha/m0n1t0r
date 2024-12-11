pub type Result<T> = std::result::Result<T, Error>;

use remoc::{rch::ConnectError, rtc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("remote call error: {0}")]
    RtcError(#[from] rtc::CallError),

    #[error("channel connect error: {0}")]
    ChannelConnectError(#[from] ConnectError),

    #[error("procedure error: {0}")]
    IoError(serde_error::Error),

    #[error("http error: {0}")]
    HttpError(serde_error::Error),

    #[error("ffi error: {0}")]
    FfiError(serde_error::Error),

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error),

    #[error("unsupported procedure")]
    Unsupported,

    #[error("procedure unimplemented")]
    Unimplemented,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(serde_error::Error::new(&*e))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        Self::IoError(serde_error::Error::new(&e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(serde_error::Error::new(&e))
    }
}

impl From<cxx::Exception> for Error {
    fn from(e: cxx::Exception) -> Self {
        Self::FfiError(serde_error::Error::new(&e))
    }
}
