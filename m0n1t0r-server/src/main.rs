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
struct GeneralConfig {
    #[serde(default = "default_log_level")]
    log_level: String,
    secret: String,
}

#[derive(Deserialize)]
struct ConnConfig {
    #[serde(default = "default_conn_addr")]
    addr: SocketAddr,
}

impl Default for ConnConfig {
    fn default() -> Self {
        Self {
            addr: default_conn_addr(),
        }
    }
}

#[derive(Deserialize)]
struct ApiConfig {
    #[serde(default = "default_api_addr")]
    addr: SocketAddr,
    #[serde(default)]
    use_https: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            addr: default_api_addr(),
            use_https: false,
        }
    }
}

#[derive(Deserialize)]
struct TlsConfig {
    key: PathBuf,
    cert: PathBuf,
}

#[derive(Deserialize)]
struct FileConfig {
    general: GeneralConfig,
    #[serde(default)]
    conn: ConnConfig,
    #[serde(default)]
    api: ApiConfig,
    tls: TlsConfig,
}

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
