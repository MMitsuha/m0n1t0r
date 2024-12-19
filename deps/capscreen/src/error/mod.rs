pub type Result<T> = std::result::Result<T, Error>;

cfg_block! {
    #[cfg(target_os = "macos")] {
        mod macos;
        #[allow(unused_imports)]
        pub use macos::*;
    }
    #[cfg(target_os = "windows")] {
        pub(crate) mod windows;
        #[allow(unused_imports)]
        pub use windows::*;
    }
    #[cfg(target_os = "linux")] {
        pub(crate) mod linux;
        #[allow(unused_imports)]
        pub use linux::*;
    }
}

use crate::frame::Frame;
use cfg_block::cfg_block;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Detail {
    code: Option<isize>,
    message: String,
}

impl Display for Detail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.message, self.code)
    }
}

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    #[error("failed to detect availability: {0}")]
    AvailabilityDetectionFailed(Detail),
    #[error("version is not a valid semver")]
    NotSemVer,
    #[error("failed to list screen: {0}")]
    ListScreenFailed(Detail),
    #[error("failed to edit config: {0}")]
    EditConfigFailed(Detail),
    #[error("failed to get frame: {0}")]
    FrameNotReceived(Detail),
    #[error("failed to send frame: {0}")]
    FrameNotSent(Detail),
    #[error("failed to parse frame: {0}")]
    ParseFrameFailed(Detail),
    #[error("frame stopped")]
    FrameStopped,
    #[error("no display found")]
    NoDisplayFound(Detail),
    #[error("failed to start capture: {0}")]
    CaptureStartFailed(Detail),
    #[error("capturer already stopped")]
    AlreadyStopped,
    #[error("failed to stop capture: {0}")]
    CaptureStopFailed(Detail),
    #[error("not implemented")]
    NotImplemented,
}

impl From<ring_channel::RecvError> for Error {
    fn from(value: ring_channel::RecvError) -> Self {
        Self::FrameNotReceived(Detail {
            code: None,
            message: value.to_string(),
        })
    }
}

impl From<ring_channel::SendError<Frame>> for Error {
    fn from(value: ring_channel::SendError<Frame>) -> Self {
        Self::FrameNotReceived(Detail {
            code: None,
            message: value.to_string(),
        })
    }
}

impl From<&str> for Detail {
    fn from(value: &str) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}
