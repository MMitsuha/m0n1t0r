pub type Result<T> = std::result::Result<T, Error>;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("http request failed: {0}")]
    HttpRequestFailed(serde_error::Error),

    #[error("request qq failed")]
    RequestQQError,

    #[error("build regex failed: {0}")]
    RegexException(serde_error::Error),

    #[error("regex no match")]
    RegexNoMatch(String),

    #[error("parse int failed: {0}")]
    InvalidInt(serde_error::Error),

    #[error("cookie store lock poisoned: {0}")]
    CookieStoreLockPoisoned(serde_error::Error),

    #[error("cookie not found: {0}")]
    CookieNotFound(String),

    #[error("field not found: {0}")]
    FieldNotFound(String),

    #[error("invalid character")]
    InvalidCharacter,

    #[error("invalid url: {0}")]
    InvalidUrl(serde_error::Error),

    #[error("invalid url: no query")]
    UrlNoQuery,

    #[error("invalid url: no domain")]
    UrlNoDomain,

    #[error("invalid utf8 string")]
    InvalidUtf8(serde_error::Error),

    #[error("serialization failed: {0}")]
    SerializeError(serde_error::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpRequestFailed(serde_error::Error::new(&e))
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Self::RegexException(serde_error::Error::new(&e))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::InvalidInt(serde_error::Error::new(&e))
    }
}

impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, reqwest_cookie_store::CookieStore>>>
    for Error
{
    fn from(
        e: std::sync::PoisonError<
            std::sync::RwLockReadGuard<'_, reqwest_cookie_store::CookieStore>,
        >,
    ) -> Self {
        Self::CookieStoreLockPoisoned(serde_error::Error::new(&e))
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::InvalidUrl(serde_error::Error::new(&e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8(serde_error::Error::new(&e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerializeError(serde_error::Error::new(&e))
    }
}
