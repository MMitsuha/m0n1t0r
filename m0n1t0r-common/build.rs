use m0n1t0r_build::{cert, version};

fn main() {
    version::generate();
    cert::ensure();
}
