pub type Result<T> = std::result::Result<T, Error>;

use crate::web::Response;
use actix_web::{http::StatusCode, HttpResponse};
use m0n1t0r_common::util::Discriminant;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone, Discriminant)]
#[repr(i16)]
pub enum Error {
    #[error("operation succeeded")]
    Okay = 0,

    #[error("serialization failed: {0}")]
    SerializeError(serde_error::Error) = -1,

    #[error("specified object not found")]
    NotFound = -2,

    #[error("remote call failed with exception: {0}")]
    RtcException(m0n1t0r_common::Error) = -3,

    #[error("web framework failed with exception: {0}")]
    WebFrameworkException(serde_error::Error) = -4,

    #[error("channel connect failed with exception: {0}")]
    RchException(#[from] remoc::rch::ConnectError) = -5,

    #[error("parse command failed: {0}")]
    InvalidCommand(serde_error::Error) = -6,

    #[error("tokio io failed: {0}")]
    TokioIoFailed(serde_error::Error) = -7,

    #[error("invalid ip address: {0}")]
    InvalidIpAddress(serde_error::Error) = -8,

    #[error("invalid web parameter: {0}")]
    InvalidWebParameter(serde_error::Error) = -9,

    #[error("parse int failed: {0}")]
    InvalidInt(serde_error::Error) = -10,

    #[error("qqkey operation failed: {0}")]
    QQKeyException(#[from] qqkey::Error) = -11,

    #[error("socks5 auth failed")]
    Socks5AuthFailed(serde_error::Error) = -12,

    #[error("socks5 operation failed: {0}")]
    Socks5Exception(serde_error::Error) = -13,

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error) = -255,
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(Response::error(self.clone()).unwrap_or_default())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Okay => StatusCode::OK,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InvalidWebParameter(_)
            | Error::InvalidCommand(_)
            | Error::InvalidInt(_)
            | Error::InvalidIpAddress(_) => StatusCode::BAD_REQUEST,
            Error::Socks5AuthFailed(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(serde_error::Error::new(&*e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerializeError(serde_error::Error::new(&e))
    }
}

impl From<m0n1t0r_common::Error> for Error {
    fn from(e: m0n1t0r_common::Error) -> Self {
        Self::RtcException(e)
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Self::WebFrameworkException(serde_error::Error::new(&e))
    }
}

impl From<shell_words::ParseError> for Error {
    fn from(e: shell_words::ParseError) -> Self {
        Self::InvalidCommand(serde_error::Error::new(&e))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        Self::TokioIoFailed(serde_error::Error::new(&e))
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::InvalidIpAddress(serde_error::Error::new(&e))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::InvalidInt(serde_error::Error::new(&e))
    }
}

impl From<actix_web::error::PathError> for Error {
    fn from(e: actix_web::error::PathError) -> Self {
        Self::InvalidWebParameter(serde_error::Error::new(&e))
    }
}

impl From<actix_web::error::QueryPayloadError> for Error {
    fn from(e: actix_web::error::QueryPayloadError) -> Self {
        Self::InvalidWebParameter(serde_error::Error::new(&e))
    }
}

impl From<actix_web::error::UrlencodedError> for Error {
    fn from(e: actix_web::error::UrlencodedError) -> Self {
        Self::InvalidWebParameter(serde_error::Error::new(&e))
    }
}

impl From<socks5_impl::Error> for Error {
    fn from(e: socks5_impl::Error) -> Self {
        Self::Socks5Exception(serde_error::Error::new(&e))
    }
}
