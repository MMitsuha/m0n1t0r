#[cfg(debug_assertions)]
pub mod debug;

use anyhow::{Result, anyhow};
use m0n1t0r_common::{
    client::{Client, ClientClient},
    server::Server,
};
use remoc::rtc;
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;

pub struct ServerObj {
    addr: SocketAddr,
    canceller: CancellationToken,
    client_client: Option<ClientClient>,
}

impl ServerObj {
    pub fn new(addr: &SocketAddr) -> Self {
        let canceller = CancellationToken::new();

        Self {
            addr: *addr,
            canceller,
            client_client: None,
        }
    }

    pub fn initialize(&mut self, client_client: ClientClient) {
        self.client_client = Some(client_client);
    }

    pub fn canceller(&self) -> CancellationToken {
        self.canceller.clone()
    }

    pub fn client(&self) -> Result<&ClientClient> {
        self.client_client
            .as_ref()
            .ok_or(anyhow!("client not connected"))
    }

    pub async fn terminate(&self) -> Result<()> {
        let _ = self.client()?.terminate().await;
        self.canceller().cancel();
        Ok(())
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}

#[rtc::async_trait]
impl Server for ServerObj {}
