use crate::{fs, info, network, process, proxy, qq, util, Result as AppResult};
use remoc::rtc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, path::PathBuf};
use tokio::fs as tfs;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TargetPlatform {
    General,
    Windows,
    Linux,
    MacOS,
}

#[rtc::remote]
pub trait Client: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::version().into())
    }

    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::General)
    }

    async fn system_info(&self) -> AppResult<info::System> {
        Ok(info::System::new())
    }

    async fn build_time(&self) -> AppResult<String> {
        Ok(util::version::build_time().into())
    }

    async fn commit_hash(&self) -> AppResult<String> {
        Ok(util::version::commit_hash().into())
    }

    async fn terminate(&self) -> AppResult<()>;

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }

    async fn get_fs_agent(&self) -> AppResult<fs::AgentClient>;

    async fn get_process_agent(&self) -> AppResult<process::AgentClient>;

    async fn get_proxy_agent(&self) -> AppResult<proxy::AgentClient>;

    async fn get_network_agent(&self) -> AppResult<network::AgentClient>;

    async fn get_qq_agent(&self) -> AppResult<qq::AgentClient>;

    async fn update_by_url(&self, url: Url, temp: PathBuf) -> AppResult<()> {
        util::network::download(url, &temp).await?;
        util::update::update_internal(temp).await
    }

    async fn update_by_file(&self, file: Vec<u8>, temp: PathBuf) -> AppResult<()> {
        tfs::write(&temp, file).await?;
        util::update::update_internal(temp).await
    }

    async fn environment(&self) -> AppResult<HashMap<String, String>> {
        Ok(env::vars().collect())
    }
}
