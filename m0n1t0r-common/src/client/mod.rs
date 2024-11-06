use crate::{fs, process, proxy, util, Result as AppResult};
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

    async fn get_file_agent(&self) -> AppResult<fs::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;

    async fn get_proxy_agent(&self) -> AppResult<proxy::AgentClient>;
}
