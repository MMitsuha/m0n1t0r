pub type Result<T> = std::result::Result<T, Error>;

use crate::web::Response;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

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
