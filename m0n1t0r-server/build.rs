use std::path::Path;

fn main() {
    m0n1t0r_build::platform::ensure();

    #[cfg(feature = "winnt")]
    m0n1t0r_build::winres::embed(
        &Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/ms.ico"),
        None,
    );
}
