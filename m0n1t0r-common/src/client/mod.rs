use crate::{fs, network, process, proxy, util, Result as AppResult};
use remoc::rtc;
use tokio::fs as tfs;
use url::Url;

const UPDATE_TEMP_PATH: &str = "tmp.bin";

#[rtc::remote]
pub trait Client: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::get())
    }

    async fn target_platform(&self) -> AppResult<String> {
        Ok("general".to_string())
    }

    async fn terminate(&self) -> AppResult<()>;

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }

    async fn get_file_agent(&self) -> AppResult<fs::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;

    async fn get_proxy_agent(&self) -> AppResult<proxy::AgentClient>;

    async fn get_network_agent(&self) -> AppResult<network::AgentClient>;

    async fn update(&self, url: Url) -> AppResult<()> {
        util::network::download(url, UPDATE_TEMP_PATH.into()).await?;
        self_replace::self_replace(UPDATE_TEMP_PATH)?;
        tfs::remove_file(UPDATE_TEMP_PATH).await?;
        Ok(())
    }
}
