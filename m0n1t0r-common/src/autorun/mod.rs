use crate::{Error, Result as AppResult};
use remoc::rtc;
use std::path::PathBuf;

#[rtc::remote]
pub trait Agent: Sync {
    async fn add_current_user_bashrc(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user(&self, _file: PathBuf) -> AppResult<()> {
        Err(Error::Unsupported)
    }
}
