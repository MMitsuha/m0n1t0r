pub type Result<T> = std::result::Result<T, Error>;

use crate::web::Response;
use actix_web::{
    error::{PathError, QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use remoc::rch::ConnectError;
use serde::Serialize;
use shell_words::ParseError;
use std::{net::AddrParseError, num::ParseIntError};
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug, Serialize, Clone)]
#[repr(i16)]
pub enum Error {
    #[error("operation succeeded")]
    Okay = 0,

    #[error("serialization error: {0}")]
    SerializeError(serde_error::Error) = -1,

    #[error("can not find specified client")]
    ClientNotFound = -2,

    #[error("remote call error: {0}")]
    RemoteCallError(m0n1t0r_common::Error) = -3,

    #[error("web framework error: {0}")]
    WebFrameworkError(serde_error::Error) = -4,

    #[error("channel connect error: {0}")]
    ChannelConnectError(#[from] ConnectError) = -5,

    #[error("parse command error: {0}")]
    ParseCommandError(serde_error::Error) = -6,

    #[error("io error: {0}")]
    IoError(serde_error::Error) = -7,

    #[error("parse addr error: {0}")]
    ParseAddrError(serde_error::Error) = -8,

    #[error("extractor error: {0}")]
    ExtractorError(serde_error::Error) = -9,

    #[error("parse int error: {0}")]
    ParseIntError(serde_error::Error) = -10,

    #[error("unknown error: {0}")]
    Unknown(serde_error::Error) = -255,
}

impl Error {
    pub fn discriminant(&self) -> i16 {
        unsafe { *(self as *const Self as *const i16) }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(Response::error(self.clone()).unwrap_or_default())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Okay => StatusCode::OK,
            Error::ClientNotFound => StatusCode::NOT_FOUND,
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
        Self::RemoteCallError(e)
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Self::WebFrameworkError(serde_error::Error::new(&e))
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::ParseCommandError(serde_error::Error::new(&e))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(serde_error::Error::new(&e))
    }
}

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Self::ParseAddrError(serde_error::Error::new(&e))
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(serde_error::Error::new(&e))
    }
}

impl From<PathError> for Error {
    fn from(e: PathError) -> Self {
        Self::ExtractorError(serde_error::Error::new(&e))
    }
}

impl From<QueryPayloadError> for Error {
    fn from(e: QueryPayloadError) -> Self {
        Self::ExtractorError(serde_error::Error::new(&e))
    }
}
