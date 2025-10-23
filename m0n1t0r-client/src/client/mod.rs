#[macro_use]
mod r#macro;

mod general;

#[cfg(any(feature = "linux", feature = "macos"))]
mod unix;

#[cfg(feature = "windows")]
mod windows;

use chrono::{DateTime, Local};
use m0n1t0r_common::{
    Result as AppResult,
    client::{Client, TargetPlatform},
    server::ServerClient,
    util::time,
};
use remoc::{prelude::ServerSharedMut, rtc};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

declare_agents!(
    general,
    [proxy, network, qq],
    ["general", "macos", "linux", "windows"]
);
declare_agents!(windows, [process, autorun, charset, fs], ["windows"]);
declare_agents!(
    general,
    [process, charset, fs],
    ["general", "macos", "linux"]
);
declare_agents!(unix, [autorun], ["linux", "macos"]);
declare_agents!(general, [autorun], ["general"]);

pub struct ClientObj {
    #[allow(dead_code)]
    addr: SocketAddr,
    canceller: CancellationToken,
    server_client: Option<ServerClient>,
    terminator: CancellationToken,
    time: DateTime<Local>,
}

impl ClientObj {
    pub fn new(addr: &SocketAddr) -> Self {
        Self {
            addr: *addr,
            canceller: CancellationToken::new(),
            server_client: None,
            terminator: CancellationToken::new(),
            time: time::local(),
        }
    }

    pub fn initialize(&mut self, server_client: ServerClient) {
        self.server_client = Some(server_client);
    }

    pub fn canceller(&self) -> CancellationToken {
        self.canceller.clone()
    }

    pub fn terminator(&self) -> CancellationToken {
        self.terminator.clone()
    }

    fn target_platform_internal() -> TargetPlatform {
        if cfg!(feature = "windows") {
            TargetPlatform::Windows
        } else if cfg!(feature = "linux") {
            TargetPlatform::Linux
        } else if cfg!(feature = "macos") {
            TargetPlatform::MacOS
        } else {
            TargetPlatform::General
        }
    }
}

#[rtc::async_trait]
impl Client for ClientObj {
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(Self::target_platform_internal())
    }

    async fn terminate(&self) -> AppResult<()> {
        self.terminator.cancel();
        Ok(())
    }

    async fn connected_time(&self) -> AppResult<DateTime<Local>> {
        Ok(self.time)
    }

    async fn fs_agent(&self) -> AppResult<m0n1t0r_common::fs::AgentClient> {
        create_agent_instance!(fs)
    }

    async fn process_agent(&self) -> AppResult<m0n1t0r_common::process::AgentClient> {
        create_agent_instance!(process)
    }

    async fn proxy_agent(&self) -> AppResult<m0n1t0r_common::proxy::AgentClient> {
        create_agent_instance!(proxy)
    }

    async fn network_agent(&self) -> AppResult<m0n1t0r_common::network::AgentClient> {
        create_agent_instance!(network)
    }

    async fn qq_agent(&self) -> AppResult<m0n1t0r_common::qq::AgentClient> {
        create_agent_instance!(qq)
    }

    async fn autorun_agent(&self) -> AppResult<m0n1t0r_common::autorun::AgentClient> {
        create_agent_instance!(autorun)
    }

    async fn charset_agent(&self) -> AppResult<m0n1t0r_common::charset::AgentClient> {
        create_agent_instance!(charset)
    }
}
