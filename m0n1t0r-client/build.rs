use std::{path::PathBuf, process::Command};

#[allow(warnings)]
const PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];

fn xmake_build() -> Vec<PathBuf> {
    #[allow(warnings)]
    let mut paths = Vec::new();

    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");

    #[cfg(feature = "windows")]
    paths.append(
        &mut PROJECT_LIST_WINDOWS
            .iter()
            .map(|x| xmake::build(format!("{}", x)))
            .collect::<Vec<PathBuf>>(),
    );

    paths
}

fn main() {
    let _ = cxx_build::bridge("src/client/windows/process.rs");

    for path in xmake_build() {
        println!("cargo:rustc-link-search={}", path.display());
    }

    #[cfg(feature = "windows")]
    PROJECT_LIST_WINDOWS.iter().for_each(|x| {
        println!("cargo:rustc-link-lib={}", x);
    });
}
