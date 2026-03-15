use anyhow::{Context, Result};
use flexi_logger::Logger;
use m0n1t0r_build::config::FileConfig;
use m0n1t0r_server::{Config, ServerMap};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::args().nth(1).unwrap_or("config.toml".into());
    let content =
        std::fs::read_to_string(&config_path).context(format!("failed to read {config_path}"))?;
    let file_config: FileConfig =
        toml::from_str(&content).context(format!("failed to parse {config_path}"))?;

    Logger::try_with_str(&file_config.general.log_level)?.start()?;
    #[cfg(feature = "rd")]
    ffmpeg_next::init()?;

    let config = Config::new(
        &file_config.conn.addr,
        &file_config.api.addr,
        &file_config.tls.key,
        &file_config.tls.cert,
        file_config.api.use_https,
        file_config.general.secret,
    )?;
    let server_map = Arc::new(RwLock::new(ServerMap::new()));

    m0n1t0r_server::run(&config, server_map).await?;
    Ok(())
}
