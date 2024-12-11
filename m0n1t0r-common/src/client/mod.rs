use crate::{fs, info, network, process, proxy, screen, util, Result as AppResult};
use remoc::rtc;
use serde::{Deserialize, Serialize};
use tokio::fs as tfs;
use url::Url;

const UPDATE_TEMP_PATH: &str = "tmp.bin";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TargetPlatform {
    General,
    Windows,
}

#[rtc::remote]
pub trait Client: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::get())
    }

    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::General)
    }

    async fn system_info(&self) -> AppResult<info::System> {
        Ok(info::System::new())
    }

    async fn terminate(&self) -> AppResult<()>;

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }

    async fn get_fs_agent(&self) -> AppResult<fs::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;

    async fn get_proxy_agent(&self) -> AppResult<proxy::AgentClient>;

    async fn get_network_agent(&self) -> AppResult<network::AgentClient>;

    async fn get_screen_agent(&self) -> AppResult<screen::AgentClient>;

    async fn update(&self, url: Url) -> AppResult<()> {
        util::network::download(url, UPDATE_TEMP_PATH.into()).await?;
        self_replace::self_replace(UPDATE_TEMP_PATH)?;
        tfs::remove_file(UPDATE_TEMP_PATH).await?;
        Ok(())
    }
}
