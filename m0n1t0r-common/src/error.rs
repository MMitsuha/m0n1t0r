pub type Result<T> = std::result::Result<T, Error>;

use remoc::rtc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("remote call error: {0}")]
    RtcError(#[from] rtc::CallError),

    #[error("procedure error: {0}")]
    ProcedureError(serde_error::Error),

    #[error("procedure unimplemented")]
    Unimplemented,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::ProcedureError(serde_error::Error::new(&*e))
    }
}
