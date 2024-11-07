pub type Result<T> = std::result::Result<T, Error>;

use remoc::{rch::ConnectError, rtc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

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

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error),

    #[error("procedure unimplemented")]
    Unimplemented,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(serde_error::Error::new(&*e))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(serde_error::Error::new(&e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(serde_error::Error::new(&e))
    }
}
