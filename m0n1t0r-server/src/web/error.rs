pub type Result<T> = std::result::Result<T, Error>;

use crate::web::Response;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone, Copy)]
#[repr(i16)]
pub enum Error {
    #[error("operation succeeded")]
    Okay = 0,
    #[error("serialization error")]
    SerializeError = -1,
    #[error("can not find specified client")]
    ClientNotFound = -2,
    #[error("unknown error")]
    Unknown = -255,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(Response::error(*self).unwrap_or_default())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Okay => StatusCode::OK,
            Error::ClientNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
