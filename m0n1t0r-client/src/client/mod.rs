#[macro_use]
mod r#macro;

mod general;

#[cfg(any(feature = "linux", feature = "macos"))]
mod unix;

#[cfg(feature = "windows")]
mod windows;

#[cfg(any(feature = "windows", feature = "linux", feature = "macos"))]
use m0n1t0r_common::client::TargetPlatform;

use log::warn;
use m0n1t0r_common::{Result as AppResult, client::Client, server::ServerClient};
use remoc::{prelude::ServerSharedMut, rtc};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

declare_agents!(
    general,
    [proxy, network, fs, qq],
    ["general", "macos", "linux", "windows"]
);
declare_agents!(windows, [process], ["windows"]);
declare_agents!(general, [process], ["general", "macos", "linux"]);
declare_agents!(unix, [autorun], ["linux", "macos"]);
declare_agents!(general, [autorun], ["general", "windows"]);

pub struct ClientObj {
    addr: SocketAddr,
    canceller: CancellationToken,
    server_client: Option<ServerClient>,
    terminator: CancellationToken,
}

impl ClientObj {
    pub fn new(addr: &SocketAddr) -> Self {
        Self {
            addr: addr.clone(),
            canceller: CancellationToken::new(),
            server_client: None,
            terminator: CancellationToken::new(),
        }
    }

    pub fn initialize(&mut self, server_client: ServerClient) {
        self.server_client = Some(server_client);
    }

    pub fn get_canceller(&self) -> CancellationToken {
        self.canceller.clone()
    }

    pub fn get_terminator(&self) -> CancellationToken {
        self.terminator.clone()
    }
}

#[rtc::async_trait]
impl Client for ClientObj {
    #[cfg(feature = "windows")]
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::Windows)
    }

    #[cfg(feature = "linux")]
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::Linux)
    }

    #[cfg(feature = "macos")]
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::MacOS)
    }

    async fn terminate(&self) -> AppResult<()> {
        self.terminator.cancel();
        Ok(())
    }

    async fn get_fs_agent(&self) -> AppResult<m0n1t0r_common::fs::AgentClient> {
        impl_agent!(fs, self)
    }

    async fn get_process_agent(&self) -> AppResult<m0n1t0r_common::process::AgentClient> {
        impl_agent!(process, self)
    }

    async fn get_proxy_agent(&self) -> AppResult<m0n1t0r_common::proxy::AgentClient> {
        impl_agent!(proxy, self)
    }

    async fn get_network_agent(&self) -> AppResult<m0n1t0r_common::network::AgentClient> {
        impl_agent!(network, self)
    }

    async fn get_qq_agent(&self) -> AppResult<m0n1t0r_common::qq::AgentClient> {
        impl_agent!(qq, self)
    }

    async fn get_autorun_agent(&self) -> AppResult<m0n1t0r_common::autorun::AgentClient> {
        impl_agent!(autorun, self)
    }
}
