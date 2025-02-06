pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn build_time() -> &'static str {
    env!("VERGEN_BUILD_TIMESTAMP")
}

pub fn commit_hash() -> &'static str {
    env!("VERGEN_RUSTC_COMMIT_HASH")
}
