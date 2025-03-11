mod client;
mod conn;

pub use client::ClientObj;
pub use conn::ClientMap;

use anyhow::Result;
use log::warn;
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time};

pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
}

pub async fn run(config: &Config, client_map: Arc<RwLock<ClientMap>>) -> Result<()> {
    while let Err(e) = conn::run(
        &conn::Config::from_crate_config(config).await?,
        client_map.clone(),
    )
    .await
    {
        warn!("connection error: {}", e);
        time::sleep(Duration::from_secs(10)).await;
    }

    Ok(())
}
