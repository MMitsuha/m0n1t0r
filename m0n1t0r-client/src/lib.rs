mod client;
mod conn;

pub use client::ClientObj;
pub use conn::ClientMap;

use anyhow::Result;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: SocketAddr,
    client_map: Arc<RwLock<ClientMap>>,
}

impl Config {
    pub fn new(addr: &SocketAddr) -> Self {
        Self {
            addr: addr.clone(),
            client_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn run(config: &Config) -> Result<()> {
    conn::run(&config.into()).await?;
    Ok(())
}
