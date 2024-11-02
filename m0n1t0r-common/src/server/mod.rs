use std::time::Duration;

use crate::{util, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Server: Sync {
    async fn ping(&self, before: Duration) -> AppResult<Duration> {
        Ok(util::time::ping(before).await?)
    }
}
