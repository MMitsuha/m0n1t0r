use std::path::{Path, PathBuf};

const CA_CERT: &str = "ca.crt";
const END_KEY: &str = "end.key";
const END_CERT: &str = "end.crt";

pub fn path() -> PathBuf {
    Path::new(env!("CARGO_WORKSPACE_DIR")).join("certs")
}

pub fn check(certs: &Path) -> bool {
    cargo_emit::rerun_if_changed!(certs.display());
    check_no_rerun(certs)
}

pub fn check_no_rerun(certs: &Path) -> bool {
    ![
        certs.join(CA_CERT),
        certs.join(END_KEY),
        certs.join(END_CERT),
    ]
    .into_iter()
    .any(|p| !p.exists())
}
