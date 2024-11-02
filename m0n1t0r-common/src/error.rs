use remoc::rtc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("remote call error")]
    RtcError(rtc::CallError),

    #[error("remote call error")]
    ProcedureError(),
}

impl From<rtc::CallError> for Error {
    fn from(e: rtc::CallError) -> Self {
        Self::RtcError(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::ProcedureError()
    }
}
