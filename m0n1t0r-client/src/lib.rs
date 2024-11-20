mod client;
mod conn;

pub use client::ClientObj;
pub use conn::ClientMap;

use anyhow::Result;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    host: String,
    addr: SocketAddr,
}

impl Config {
    pub fn new(host: &str) -> Result<Self> {
        Ok(Self {
            host: host.to_string(),
            addr: host.parse()?,
        })
    }
}

pub async fn run(config: &Config, client_map: Arc<RwLock<ClientMap>>) -> Result<()> {
    conn::run(&config.into(), client_map).await?;
    Ok(())
}
