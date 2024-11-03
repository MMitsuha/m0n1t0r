use crate::{file, process, util, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Client: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::get())
    }

    async fn terminate(&self) -> AppResult<()>;

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }

    async fn get_file_agent(&self) -> AppResult<file::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;
}
