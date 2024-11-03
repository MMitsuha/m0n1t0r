pub type Result<T> = std::result::Result<T, Error>;

use remoc::rtc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("remote call error")]
    RtcError(#[from] rtc::CallError),

    #[error("remote call error")]
    ProcedureError,
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::ProcedureError
    }
}
