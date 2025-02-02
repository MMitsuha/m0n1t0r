#[macro_use]
mod agent;

#[cfg(feature = "windows")]
mod windows;

#[cfg(any(feature = "windows", feature = "linux", feature = "macos"))]
use m0n1t0r_common::client::TargetPlatform;

use m0n1t0r_common::{client::Client, server::ServerClient, Result as AppResult};
use remoc::{prelude::ServerSharedMut, rtc};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

declare_agents!(proxy, network, fs);
declare_agents_with_platform!("windows", process);

pub struct ClientObj {
    _addr: SocketAddr,
    canceller: CancellationToken,
    server_client: Option<ServerClient>,
    terminator: CancellationToken,
}

impl ClientObj {
    pub fn new(addr: &SocketAddr) -> Self {
        Self {
            _addr: addr.clone(),
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
    async fn terminate(&self) -> AppResult<()> {
        self.terminator.cancel();
        Ok(())
    }

    #[cfg(feature = "windows")]
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::Windows)
    }

    async fn get_fs_agent(&self) -> AppResult<m0n1t0r_common::fs::AgentClient> {
        impl_agent!(fs)
    }

    async fn get_process_agent(&self) -> AppResult<m0n1t0r_common::process::AgentClient> {
        impl_agent!(process)
    }

    async fn get_proxy_agent(&self) -> AppResult<m0n1t0r_common::proxy::AgentClient> {
        impl_agent!(proxy)
    }

    async fn get_network_agent(&self) -> AppResult<m0n1t0r_common::network::AgentClient> {
        impl_agent!(network)
    }
}
