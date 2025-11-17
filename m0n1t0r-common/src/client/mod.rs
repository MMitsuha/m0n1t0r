use crate::{
    Result as AppResult, autorun, blind, charset, fs, info, network, process, proxy, qq, rd,
    util::{self, shell::Shell},
};
use chrono::{DateTime, Local};
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
    async fn shell(&self) -> AppResult<Shell> {
        Ok(Shell::new())
    }

    async fn version(&self) -> AppResult<String> {
        Ok(util::version::version().into())
    }

    async fn target_platform(&self) -> AppResult<TargetPlatform>;

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

    async fn fs_agent(&self) -> AppResult<fs::AgentClient>;

    async fn process_agent(&self) -> AppResult<process::AgentClient>;

    async fn proxy_agent(&self) -> AppResult<proxy::AgentClient>;

    async fn network_agent(&self) -> AppResult<network::AgentClient>;

    async fn qq_agent(&self) -> AppResult<qq::AgentClient>;

    async fn autorun_agent(&self) -> AppResult<autorun::AgentClient>;

    async fn charset_agent(&self) -> AppResult<charset::AgentClient>;

    async fn rd_agent(&self) -> AppResult<rd::AgentClient>;

    async fn blind_agent(&self) -> AppResult<blind::AgentClient>;

    async fn update_by_url(&self, url: Url, temp: PathBuf) -> AppResult<()> {
        util::network::download(url, &temp).await?;
        util::update::update(temp).await
    }

    async fn update_by_file(&self, file: Vec<u8>, temp: PathBuf) -> AppResult<()> {
        tfs::write(&temp, file).await?;
        util::update::update(temp).await
    }

    async fn environment(&self) -> AppResult<HashMap<String, String>> {
        Ok(env::vars().collect())
    }

    async fn current_exe(&self) -> AppResult<PathBuf> {
        Ok(env::current_exe()?.to_path_buf())
    }

    async fn connected_time(&self) -> AppResult<DateTime<Local>>;
}
