use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};

fn default_conn_addr() -> SocketAddr {
    "0.0.0.0:27853".parse().unwrap()
}

fn default_api_addr() -> SocketAddr {
    "0.0.0.0:10801".parse().unwrap()
}

fn default_log_level() -> String {
    "debug".into()
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConnConfig {
    #[serde(default = "default_conn_addr")]
    pub addr: SocketAddr,
}

impl Default for ConnConfig {
    fn default() -> Self {
        Self {
            addr: default_conn_addr(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_api_addr")]
    pub addr: SocketAddr,
    #[serde(default)]
    pub use_https: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            addr: default_api_addr(),
            use_https: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TlsConfig {
    pub key: PathBuf,
    pub cert: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct CertConfig {
    pub country: String,
    pub state: String,
    pub locality: String,
    pub org: String,
    pub unit: String,
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    pub general: GeneralConfig,
    #[serde(default)]
    pub conn: ConnConfig,
    #[serde(default)]
    pub api: ApiConfig,
    pub tls: TlsConfig,
    pub cert: CertConfig,
}
