mod client;
mod conn;

pub use client::ClientObj;
pub use conn::ClientMap;

use anyhow::Result;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};
use tokio::sync::RwLock;

pub struct Config {
    host: String,
    addr: SocketAddr,
}

impl Config {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            host: host.to_string(),
            addr: (host, port)
                .to_socket_addrs()?
                .next()
                .ok_or(anyhow::anyhow!("no address found"))?,
        })
    }
}

pub async fn run(config: &Config, client_map: Arc<RwLock<ClientMap>>) -> Result<()> {
    conn::run(&config.into(), client_map).await?;
    Ok(())
}
