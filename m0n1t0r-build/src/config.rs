use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct CertSection {
    domain: String,
}

#[derive(Deserialize)]
struct ConfigFile {
    cert: CertSection,
}

pub fn path() -> PathBuf {
    Path::new(env!("CARGO_WORKSPACE_DIR")).join("config.toml")
}

pub fn check(config: &Path) -> bool {
    cargo_emit::rerun_if_changed!(config.display());
    check_no_rerun(config)
}

pub fn check_no_rerun(config: &Path) -> bool {
    if !config.exists() {
        return false;
    }
    let content = match std::fs::read_to_string(config) {
        Ok(c) => c,
        Err(_) => return false,
    };
    toml::from_str::<ConfigFile>(&content).is_ok()
}

pub fn domain(config: &Path) -> String {
    let content = std::fs::read_to_string(config)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", config.display()));
    let parsed: ConfigFile = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", config.display()));
    parsed.cert.domain
}
