pub type Result<T> = std::result::Result<T, Error>;

use remoc::{rch::ConnectError, rtc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

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

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::ProcedureError(serde_error::Error::new(&e))
    }
}

impl From<ConnectError> for Error {
    fn from(e: ConnectError) -> Self {
        Self::ProcedureError(serde_error::Error::new(&e))
    }
}
