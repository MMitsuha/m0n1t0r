use crate::config;
use std::path::PathBuf;

pub fn path() -> [PathBuf; 2] {
    let config = config::read();
    [config.tls.cert, config.tls.key]
}

pub fn ensure() {
    let certs = path();
    for cert in &certs {
        cargo_emit::rerun_if_changed!(cert.display());
        if !check() {
            panic!(
                "No certificate {} found. Please run `cargo xtask -c` to generate one.",
                cert.display()
            );
        }
    }
}

pub fn check() -> bool {
    path().into_iter().any(|cert| !cert.exists())
}
