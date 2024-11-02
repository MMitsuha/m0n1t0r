use std::time::Duration;

use crate::{file, process, util, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Client: Sync {
    async fn terminate(&self) -> AppResult<()>;

    async fn ping(&self, before: Duration) -> AppResult<Duration> {
        Ok(util::time::ping(before).await?)
    }

    async fn get_file_agent(&self) -> AppResult<file::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;
}
