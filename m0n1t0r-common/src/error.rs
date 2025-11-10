pub type Result<T> = std::result::Result<T, Error>;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("failed to call remote function over remoc: {0}")]
    RtcError(#[from] remoc::rtc::CallError),

    #[error("remoc channel disconnected: {0}")]
    RchDisconnected(#[from] remoc::rch::ConnectError),

    #[error("failed to send over remoc channel: {0}")]
    RchSendError(remoc::rch::lr::SendErrorKind),

    #[error("tokio io error: {0}")]
    TokioIoError(serde_error::Error),

    #[error("http request error: {0}")]
    HttpRequestError(serde_error::Error),

    #[error("foreign function call error: {0}")]
    FfiException(serde_error::Error),

    #[error("protobuf error: {0}")]
    ProtobufError(serde_error::Error),

    #[error("qqkey operation failed: {0}")]
    QQKeyError(#[from] qqkey::Error),

    #[error("specified object not found")]
    NotFound,

    #[error("invalid parameter")]
    InvalidParameter,

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error),

    #[error("invalid user directory")]
    InvalidUserDirectory,

    #[error("invalid environment value")]
    InvalidEnvironmentValue(serde_error::Error),

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
        Self::TokioIoError(serde_error::Error::new(&e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpRequestError(serde_error::Error::new(&e))
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
        Self::InvalidEnvironmentValue(serde_error::Error::new(&e))
    }
}

impl<T> From<remoc::rch::lr::SendError<T>> for Error {
    fn from(e: remoc::rch::lr::SendError<T>) -> Self {
        Self::RchSendError(e.kind)
    }
}

impl From<hbb_common::protobuf::Error> for Error {
    fn from(e: hbb_common::protobuf::Error) -> Self {
        Self::ProtobufError(serde_error::Error::new(&e))
    }
}
