use crate::{Error, Result as AppResult};
use remoc::rtc;
use std::path::PathBuf;

/// Auto select shell in unix-like os
///
/// powershell in windows
#[rtc::remote]
pub trait Agent: Sync {
    async fn exist_current_user(&self) -> AppResult<bool> {
        Err(Error::Unsupported)
    }

    async fn remove_current_user(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user_at(&self, _exe: PathBuf) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }
}
