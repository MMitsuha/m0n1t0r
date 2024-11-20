#![windows_subsystem = "windows"]

use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_client::Config;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;

    let client_map = Arc::new(RwLock::new(HashMap::new()));
    let config = if cfg!(debug_assertions) {
        Config::new("127.0.0.1:27853")
    } else {
        Config::new("mmitsuha.xyz:27853")
    }?;

    m0n1t0r_client::run(&config, client_map).await?;
    Ok(())
}
