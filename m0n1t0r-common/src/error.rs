pub type Result<T> = std::result::Result<T, Error>;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("remote call with exception: {0}")]
    RtcException(#[from] remoc::rtc::CallError),

    #[error("channel disconnected: {0}")]
    ChannelDisconnected(#[from] remoc::rch::ConnectError),

    #[error("tokio io failed: {0}")]
    TokioIoFailed(serde_error::Error),

    #[error("http request failed: {0}")]
    HttpRequestFailed(serde_error::Error),

    #[error("foreign function call failed: {0}")]
    FfiException(serde_error::Error),

    #[error("api call failed: {0}")]
    ApiCallException(serde_error::Error),

    #[error("qqkey operation failed: {0}")]
    QQKeyException(#[from] qqkey::Error),

    #[error("specified object not found")]
    NotFound,

    #[error("invalid parameter")]
    InvalidParameter,

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error),

    #[error("bad os string")]
    BadOsString,

    #[error("bad user directory")]
    BadUserDirectory,

    #[error("bad environment value")]
    BadEnvironmentValue(serde_error::Error),

    #[error("unsupported operation")]
    Unsupported,

    #[error("operation unimplemented")]
    Unimplemented,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(serde_error::Error::new(&*e))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        Self::TokioIoFailed(serde_error::Error::new(&e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpRequestFailed(serde_error::Error::new(&e))
    }
}

impl From<cxx::Exception> for Error {
    fn from(e: cxx::Exception) -> Self {
        Self::FfiException(serde_error::Error::new(&e))
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::FfiException(serde_error::Error::new(&e))
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::BadEnvironmentValue(serde_error::Error::new(&e))
    }
}
