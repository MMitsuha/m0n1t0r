use anyhow::{Context, Result};
use flexi_logger::Logger;
use m0n1t0r_server::{Config, ServerMap};
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

fn default_conn_addr() -> SocketAddr {
    "0.0.0.0:27853".parse().unwrap()
}

fn default_api_addr() -> SocketAddr {
    "0.0.0.0:10801".parse().unwrap()
}

fn default_log_level() -> String {
    "debug".into()
}

#[derive(Deserialize)]
struct FileConfig {
    #[serde(default = "default_conn_addr")]
    conn_addr: SocketAddr,
    #[serde(default = "default_api_addr")]
    api_addr: SocketAddr,
    key: PathBuf,
    cert: PathBuf,
    #[serde(default)]
    use_https: bool,
    #[serde(default = "default_log_level")]
    log_level: String,
    secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::args().nth(1).unwrap_or("config.toml".into());
    let content =
        std::fs::read_to_string(&config_path).context(format!("failed to read {config_path}"))?;
    let file_config: FileConfig =
        toml::from_str(&content).context(format!("failed to parse {config_path}"))?;

    Logger::try_with_str(&file_config.log_level)?.start()?;
    ffmpeg_next::init()?;

    let config = Config::new(
        &file_config.conn_addr,
        &file_config.api_addr,
        &file_config.key,
        &file_config.cert,
        file_config.use_https,
        file_config.secret,
    )?;
    let server_map = Arc::new(RwLock::new(ServerMap::new()));

    m0n1t0r_server::run(&config, server_map).await?;
    Ok(())
}
