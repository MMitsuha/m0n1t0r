use crate::{Error, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Agent: Sync {
    async fn patch_etw_event_write(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }
}
