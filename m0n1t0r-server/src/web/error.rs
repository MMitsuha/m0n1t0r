pub type Result<T> = std::result::Result<T, Error>;

use crate::web::Response;
use actix_web::{HttpResponse, http::StatusCode};
use discriminant_rs::Discriminant;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone)]
pub enum NetworkError {
    #[error("remote call error: {0}")]
    Rpc(m0n1t0r_common::Error),

    #[error("remoc channel disconnected: {0}")]
    ChannelDisconnect(remoc::rch::ConnectError),

    #[error("socks5 error: {0}")]
    Socks5(serde_error::Error),

    #[error("dns lookup failed")]
    DnsLookupFailed,

    #[error("forward channel is invalid")]
    InvalidForward,
}

#[derive(Error, Debug, Serialize, Clone)]
pub enum IoError {
    #[error("tokio io error: {0}")]
    Tokio(serde_error::Error),
}

#[derive(Error, Debug, Serialize, Clone)]
pub enum ParseError {
    #[error("serialization failed: {0}")]
    Serialize(serde_error::Error),

    #[error("parse command failed: {0}")]
    Command(serde_error::Error),

    #[error("invalid ip address: {0}")]
    IpAddress(serde_error::Error),

    #[error("invalid int value: {0}")]
    IntValue(serde_error::Error),

    #[error("invalid web parameter: {0}")]
    WebParameter(serde_error::Error),
}

#[allow(unused)]
#[derive(Error, Debug, Serialize, Clone)]
pub enum AuthError {
    #[error("unauthorized: {0}")]
    Unauthorized(serde_error::Error),

    #[error("forbidden: {0}")]
    Forbidden(serde_error::Error),

    #[error("password or username mismatch")]
    PasswordMismatch,
}

#[derive(Error, Debug, Serialize, Clone)]
pub enum MediaError {
    #[error("ffmpeg error: {0}")]
    FFmpeg(serde_error::Error),

    #[error("unsupported video frame type")]
    UnsupportedFormat,
}

#[derive(Error, Debug, Serialize, Clone)]
pub enum FrameworkError {
    #[error("web framework error: {0}")]
    Actix(serde_error::Error),
}

#[derive(Error, Debug, Serialize, Clone)]
pub enum ExternalError {
    #[error("qqkey operation failed: {0}")]
    QQKey(qqkey::Error),
}

#[allow(unused)]
#[derive(Error, Debug, Serialize, Clone)]
pub enum Error {
    #[error(transparent)]
    Network(NetworkError),

    #[error(transparent)]
    Io(IoError),

    #[error(transparent)]
    Parse(ParseError),

    #[error(transparent)]
    Auth(AuthError),

    #[error(transparent)]
    Media(MediaError),

    #[error(transparent)]
    Framework(FrameworkError),

    #[error(transparent)]
    External(ExternalError),

    #[error("operation succeeded")]
    Okay,

    #[error("specified object not found")]
    NotFound,

    #[error("unimplemented")]
    Unimplemented,

    #[error("generic error: {0}")]
    Generic(serde_error::Error),

    #[error("unknown error")]
    Unknown,
}

impl Discriminant<i16> for Error {
    fn discriminant(&self) -> i16 {
        match self {
            Error::Okay => 0,
            Error::Parse(ParseError::Serialize(_)) => -1,
            Error::NotFound => -2,
            Error::Network(NetworkError::Rpc(_)) => -3,
            Error::Framework(FrameworkError::Actix(_)) => -4,
            Error::Network(NetworkError::ChannelDisconnect(_)) => -5,
            Error::Parse(ParseError::Command(_)) => -6,
            Error::Io(IoError::Tokio(_)) => -7,
            Error::Parse(ParseError::IpAddress(_)) => -8,
            Error::Parse(ParseError::WebParameter(_)) => -9,
            Error::Parse(ParseError::IntValue(_)) => -10,
            Error::External(ExternalError::QQKey(_)) => -11,
            Error::Network(NetworkError::DnsLookupFailed) => -12,
            Error::Network(NetworkError::Socks5(_)) => -13,
            Error::Auth(AuthError::Forbidden(_)) => -14,
            Error::Auth(AuthError::Unauthorized(_)) => -15,
            Error::Generic(_) => -16,
            Error::Unimplemented => -17,
            Error::Network(NetworkError::InvalidForward) => -18,
            Error::Media(MediaError::FFmpeg(_)) => -19,
            Error::Auth(AuthError::PasswordMismatch) => -20,
            Error::Media(MediaError::UnsupportedFormat) => -21,
            Error::Unknown => -255,
        }
    }
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(Response::error(self.clone()).unwrap_or_default())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::Okay => StatusCode::OK,

            // 400 Bad Request — client sent invalid input
            Error::Parse(ParseError::WebParameter(_))
            | Error::Parse(ParseError::Command(_))
            | Error::Parse(ParseError::IntValue(_))
            | Error::Parse(ParseError::IpAddress(_)) => StatusCode::BAD_REQUEST,

            // 401 Unauthorized
            Error::Auth(AuthError::Unauthorized(_)) => StatusCode::UNAUTHORIZED,

            // 403 Forbidden
            Error::Auth(AuthError::Forbidden(_)) | Error::Auth(AuthError::PasswordMismatch) => {
                StatusCode::FORBIDDEN
            }

            // 404 Not Found
            Error::NotFound => StatusCode::NOT_FOUND,

            // 422 Unprocessable Entity — response serialization failed
            Error::Parse(ParseError::Serialize(_)) => StatusCode::UNPROCESSABLE_ENTITY,

            // 500 Internal Server Error — server-side failures
            Error::Io(IoError::Tokio(_))
            | Error::Framework(FrameworkError::Actix(_))
            | Error::Media(MediaError::FFmpeg(_))
            | Error::Media(MediaError::UnsupportedFormat)
            | Error::External(ExternalError::QQKey(_))
            | Error::Generic(_)
            | Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,

            // 501 Not Implemented
            Error::Unimplemented => StatusCode::NOT_IMPLEMENTED,

            // 502 Bad Gateway — upstream client agent errors
            Error::Network(NetworkError::Rpc(_))
            | Error::Network(NetworkError::ChannelDisconnect(_))
            | Error::Network(NetworkError::Socks5(_))
            | Error::Network(NetworkError::DnsLookupFailed)
            | Error::Network(NetworkError::InvalidForward) => StatusCode::BAD_GATEWAY,
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Generic(serde_error::Error::new(&*e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Parse(ParseError::Serialize(serde_error::Error::new(&e)))
    }
}

impl From<m0n1t0r_common::Error> for Error {
    fn from(e: m0n1t0r_common::Error) -> Self {
        Self::Network(NetworkError::Rpc(e))
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Self::Framework(FrameworkError::Actix(serde_error::Error::new(&e)))
    }
}

impl From<actix_multipart::MultipartError> for Error {
    fn from(e: actix_multipart::MultipartError) -> Self {
        Self::Framework(FrameworkError::Actix(serde_error::Error::new(&e)))
    }
}

impl From<actix_web::error::JsonPayloadError> for Error {
    fn from(e: actix_web::error::JsonPayloadError) -> Self {
        Self::Framework(FrameworkError::Actix(serde_error::Error::new(&e)))
    }
}

impl From<shell_words::ParseError> for Error {
    fn from(e: shell_words::ParseError) -> Self {
        Self::Parse(ParseError::Command(serde_error::Error::new(&e)))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        Self::Io(IoError::Tokio(serde_error::Error::new(&e)))
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::Parse(ParseError::IpAddress(serde_error::Error::new(&e)))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::Parse(ParseError::IntValue(serde_error::Error::new(&e)))
    }
}

impl From<actix_web::error::PathError> for Error {
    fn from(e: actix_web::error::PathError) -> Self {
        Self::Parse(ParseError::WebParameter(serde_error::Error::new(&e)))
    }
}

impl From<actix_web::error::QueryPayloadError> for Error {
    fn from(e: actix_web::error::QueryPayloadError) -> Self {
        Self::Parse(ParseError::WebParameter(serde_error::Error::new(&e)))
    }
}

impl From<actix_web::error::UrlencodedError> for Error {
    fn from(e: actix_web::error::UrlencodedError) -> Self {
        Self::Parse(ParseError::WebParameter(serde_error::Error::new(&e)))
    }
}

impl From<socks5_impl::Error> for Error {
    fn from(e: socks5_impl::Error) -> Self {
        Self::Network(NetworkError::Socks5(serde_error::Error::new(&e)))
    }
}

impl From<actix_identity::error::LoginError> for Error {
    fn from(e: actix_identity::error::LoginError) -> Self {
        Self::Auth(AuthError::Forbidden(serde_error::Error::new(&e)))
    }
}

impl From<actix_ws::ProtocolError> for Error {
    fn from(e: actix_ws::ProtocolError) -> Self {
        Self::Framework(FrameworkError::Actix(serde_error::Error::new(&e)))
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

#[cfg(feature = "rd")]
impl From<ffmpeg_next::Error> for Error {
    fn from(e: ffmpeg_next::Error) -> Self {
        Self::Media(MediaError::FFmpeg(serde_error::Error::new(&e)))
    }
}

impl From<actix_ws::Closed> for Error {
    fn from(e: actix_ws::Closed) -> Self {
        Self::Framework(FrameworkError::Actix(serde_error::Error::new(&e)))
    }
}

impl<T> From<remoc::rch::base::SendError<T>> for Error {
    fn from(e: remoc::rch::base::SendError<T>) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

impl From<remoc::rch::base::RecvError> for Error {
    fn from(e: remoc::rch::base::RecvError) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

#[cfg(feature = "rd")]
impl From<scrap::Error> for Error {
    fn from(e: scrap::Error) -> Self {
        Self::Media(MediaError::FFmpeg(serde_error::Error::new(&e)))
    }
}

#[cfg(feature = "rd")]
impl From<hbb_common::protobuf::Error> for Error {
    fn from(e: hbb_common::protobuf::Error) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

impl From<remoc::chmux::SendError> for Error {
    fn from(e: remoc::chmux::SendError) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

impl From<remoc::chmux::RecvError> for Error {
    fn from(e: remoc::chmux::RecvError) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

impl From<remoc::rch::lr::RecvError> for Error {
    fn from(e: remoc::rch::lr::RecvError) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}

impl<T> From<remoc::rch::lr::SendError<T>> for Error {
    fn from(e: remoc::rch::lr::SendError<T>) -> Self {
        Self::from(m0n1t0r_common::Error::from(e))
    }
}
