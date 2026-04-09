use crate::config;
use std::path::PathBuf;

pub fn paths() -> [PathBuf; 3] {
    let root = std::path::Path::new(env!("CARGO_WORKSPACE_DIR"));
    let config = config::read();
    [
        root.join(&config.tls.ca),
        root.join(&config.tls.cert),
        root.join(&config.tls.key),
    ]
}

pub fn exists() -> bool {
    paths().iter().all(|cert| cert.exists())
}

pub fn ensure() {
    for cert in &paths() {
        cargo_emit::rerun_if_changed!(cert.display());
    }
    if !exists() {
        let missing: Vec<_> = paths()
            .into_iter()
            .filter(|c| !c.exists())
            .collect();
        panic!(
            "Missing certificate(s): {}. Please run `cargo xtask -c` to generate.",
            missing
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
