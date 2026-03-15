use crate::config;
use std::path::{Path, PathBuf};

pub fn path() -> [PathBuf; 3] {
    let root = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let config = config::read();
    [
        root.join(&config.tls.ca),
        root.join(&config.tls.cert),
        root.join(&config.tls.key),
    ]
}

pub fn ensure() {
    let certs = path();
    if check() {
        let missing: Vec<_> = certs.iter().filter(|c| !c.exists()).collect();
        panic!(
            "Missing certificate(s): {}. Please run `cargo xtask -c` to generate.",
            missing
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    for cert in &certs {
        cargo_emit::rerun_if_changed!(cert.display());
    }
}

pub fn check() -> bool {
    path().into_iter().any(|cert| !cert.exists())
}
