use crate::{Error, Result as AppResult};
use remoc::rtc;
use std::path::PathBuf;

#[rtc::remote]
pub trait Agent: Sync {
    async fn add_current_user_at(&self, _exe: PathBuf) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }
}
