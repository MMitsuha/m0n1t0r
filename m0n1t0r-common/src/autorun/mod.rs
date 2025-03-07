use crate::{Error, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Agent: Sync {
    async fn add_current_user_bashrc(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }
}
