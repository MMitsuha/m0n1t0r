#[cfg(debug_assertions)]
pub mod debug;

use anyhow::{anyhow, Result};
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
            addr: addr.clone(),
            canceller,
            client_client: None,
        }
    }

    pub fn initialize(&mut self, client_client: ClientClient) {
        self.client_client = Some(client_client);
    }

    pub fn get_canceller(&self) -> CancellationToken {
        self.canceller.clone()
    }

    pub fn get_client(&self) -> Result<&ClientClient> {
        self.client_client
            .as_ref()
            .ok_or(anyhow!("client not connected"))
    }

    pub async fn terminate(&self) -> Result<()> {
        let _ = self.get_client()?.terminate().await;
        self.get_canceller().cancel();
        Ok(())
    }

    pub fn get_addr(&self) -> &SocketAddr {
        &self.addr
    }
}

#[rtc::async_trait]
impl Server for ServerObj {}
