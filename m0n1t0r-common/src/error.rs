pub type Result<T> = std::result::Result<T, Error>;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum NetworkError {
    #[error("failed to call remote function over remoc: {0}")]
    RpcCall(#[from] remoc::rtc::CallError),

    #[error("remoc channel disconnected: {0}")]
    ChannelDisconnect(#[from] remoc::rch::ConnectError),

    #[error("failed to send over remoc channel: {0}")]
    ChannelSend(remoc::rch::lr::SendErrorKind),

    #[error("http request error: {0}")]
    HttpRequest(serde_error::Error),

    #[error("channel closed")]
    ChannelClosed,

    #[error("peer connection is invalid")]
    InvalidPeer,

    #[error("connection lost")]
    ConnectionLost,

    #[error("all connection attempts failed")]
    ConnectionFailed,
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum IoError {
    #[error("tokio io error: {0}")]
    Tokio(serde_error::Error),

    #[error("invalid user directory")]
    InvalidUserDirectory,

    #[error("invalid environment value")]
    Environment(serde_error::Error),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ParseError {
    #[error("protobuf error: {0}")]
    Protobuf(serde_error::Error),

    #[error("invalid parameter")]
    InvalidParameter,

    #[error("unsupported format")]
    UnsupportedFormat,

    #[error("invalid certificate: {0}")]
    Certificate(serde_error::Error),

    #[error("invalid DNS name: {0}")]
    DnsName(serde_error::Error),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ExternalError {
    #[error("foreign function call error: {0}")]
    Ffi(serde_error::Error),

    #[error("qqkey operation failed: {0}")]
    QQKey(#[from] qqkey::Error),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error(transparent)]
    Network(NetworkError),

    #[error(transparent)]
    Io(IoError),

    #[error(transparent)]
    Parse(ParseError),

    #[error(transparent)]
    External(ExternalError),

    #[error("specified object not found")]
    NotFound,

    #[error("initialization failed")]
    InitializationFailed,

    #[error("unsupported operation")]
    Unsupported,

    #[error("operation unimplemented")]
    Unimplemented,

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error),
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(serde_error::Error::new(&*e))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        Self::Io(IoError::Tokio(serde_error::Error::new(&e)))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(NetworkError::HttpRequest(serde_error::Error::new(&e)))
    }
}

impl From<cxx::Exception> for Error {
    fn from(e: cxx::Exception) -> Self {
        Self::External(ExternalError::Ffi(serde_error::Error::new(&e)))
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::External(ExternalError::Ffi(serde_error::Error::new(&e)))
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::Io(IoError::Environment(serde_error::Error::new(&e)))
    }
}

impl<T> From<remoc::rch::lr::SendError<T>> for Error {
    fn from(e: remoc::rch::lr::SendError<T>) -> Self {
        Self::Network(NetworkError::ChannelSend(e.kind))
    }
}

#[cfg(feature = "rd")]
impl From<hbb_common::protobuf::Error> for Error {
    fn from(e: hbb_common::protobuf::Error) -> Self {
        Self::Parse(ParseError::Protobuf(serde_error::Error::new(&e)))
    }
}

impl From<remoc::rtc::CallError> for Error {
    fn from(e: remoc::rtc::CallError) -> Self {
        Self::Network(NetworkError::RpcCall(e))
    }
}

impl From<remoc::rch::ConnectError> for Error {
    fn from(e: remoc::rch::ConnectError) -> Self {
        Self::Network(NetworkError::ChannelDisconnect(e))
    }
}

impl From<qqkey::Error> for Error {
    fn from(e: qqkey::Error) -> Self {
        Self::External(ExternalError::QQKey(e))
    }
}

impl<T> From<remoc::rch::base::SendError<T>> for Error {
    fn from(_e: remoc::rch::base::SendError<T>) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl From<remoc::rch::base::RecvError> for Error {
    fn from(_e: remoc::rch::base::RecvError) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl<T> From<tokio::sync::watch::error::SendError<T>> for Error {
    fn from(_e: tokio::sync::watch::error::SendError<T>) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl From<remoc::ConnectError<std::io::Error, std::io::Error>> for Error {
    fn from(_e: remoc::ConnectError<std::io::Error, std::io::Error>) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl From<remoc::chmux::SendError> for Error {
    fn from(_e: remoc::chmux::SendError) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl From<remoc::chmux::RecvError> for Error {
    fn from(_e: remoc::chmux::RecvError) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

impl From<remoc::rch::lr::RecvError> for Error {
    fn from(_e: remoc::rch::lr::RecvError) -> Self {
        Self::Network(NetworkError::ChannelClosed)
    }
}

