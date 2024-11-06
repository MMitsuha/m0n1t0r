mod client;
mod conn;

pub use client::ClientObj;
pub use conn::ClientMap;

use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: String,
    client_map: Arc<RwLock<ClientMap>>,
}

impl Config {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            client_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn run(config: &Config) -> Result<()> {
    conn::run(&config.into()).await?;
    Ok(())
}
