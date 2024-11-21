mod fs;
mod network;
mod process;
mod proxy;

#[cfg(not(feature = "general"))]
use m0n1t0r_common::client::TargetPlatform;

use m0n1t0r_common::{
    client::Client, fs as mcfile, network as mcnetwork, process as mcprocess, proxy as mcproxy,
    server::ServerClient, Result as AppResult,
};
use remoc::{prelude::ServerSharedMut, rtc};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

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

    #[cfg(not(feature = "general"))]
    async fn target_platform(&self) -> AppResult<TargetPlatform> {
        Ok(TargetPlatform::Specific)
    }

    async fn get_file_agent(&self) -> AppResult<mcfile::AgentClient> {
        let server = Arc::new(RwLock::new(fs::AgentObj::new()));
        let (server_server, server_client) =
            mcfile::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            server_server.serve(true).await;
        });
        Ok(server_client)
    }

    async fn get_process_agent(&self) -> AppResult<mcprocess::AgentClient> {
        let server = Arc::new(RwLock::new(process::AgentObj::new()));
        let (server_server, server_client) =
            mcprocess::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            server_server.serve(true).await;
        });
        Ok(server_client)
    }

    async fn get_proxy_agent(&self) -> AppResult<mcproxy::AgentClient> {
        let server = Arc::new(RwLock::new(proxy::AgentObj::new()));
        let (server_server, server_client) =
            mcproxy::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            server_server.serve(true).await;
        });
        Ok(server_client)
    }

    async fn get_network_agent(&self) -> AppResult<mcnetwork::AgentClient> {
        let server = Arc::new(RwLock::new(network::AgentObj::new()));
        let (server_server, server_client) =
            mcnetwork::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            server_server.serve(true).await;
        });
        Ok(server_client)
    }
}
