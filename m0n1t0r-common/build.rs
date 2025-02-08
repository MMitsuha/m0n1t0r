use cargo_emit::warning;
use m0n1t0r_build::{cert, version};
use std::path::Path;

fn main() {
    let certs = Path::new(env!("CARGO_WORKSPACE_DIR")).join("certs");

    version::generate();

    if cert::check(&certs) == false {
        warning!(
            "No certificates under {} found. Regenerating.",
            certs.display()
        );
        cert::generate(&certs);
    }
}
