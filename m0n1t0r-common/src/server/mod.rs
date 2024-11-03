use crate::{util, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Server: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::get())
    }

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }
}
