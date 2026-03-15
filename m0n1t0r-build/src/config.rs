use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

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

pub fn path() -> PathBuf {
    Path::new(env!("CARGO_WORKSPACE_DIR")).join("config.toml")
}

pub fn ensure() {
    let config = path();
    cargo_emit::rerun_if_changed!(config.display());
    if !check() {
        panic!(
            "No valid config found at {}. Please run `cargo xtask -i` to generate one.",
            config.display()
        );
    }
}

pub fn check() -> bool {
    let config = path();
    if !config.exists() {
        return false;
    }
    let content = match std::fs::read_to_string(config) {
        Ok(c) => c,
        Err(_) => return false,
    };
    toml::from_str::<FileConfig>(&content).is_ok()
}

pub fn read() -> FileConfig {
    let config = path();
    let content = match std::fs::read_to_string(&config) {
        Ok(c) => c,
        Err(e) => panic!("failed to read {}: {e}", config.display()),
    };
    let parsed: FileConfig = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => panic!("failed to parse {}: {e}", config.display()),
    };
    parsed
}
