use m0n1t0r_build::{cert, version};

fn main() {
    let certs = cert::path();

    version::generate();

    if !cert::check(&certs) {
        panic!(
            "No certificates under {} found. Please run `cargo xtask -c` to generate one.",
            certs.display()
        );
    }
}
