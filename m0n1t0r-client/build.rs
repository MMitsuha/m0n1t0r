#![allow(unused)]

#[cfg(not(any(
    feature = "windows",
    feature = "linux",
    feature = "macos",
    feature = "general"
)))]
compile_error!("No target platform specified.");

use m0n1t0r_build::{cert, dep};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const XMAKE_PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];
const BRIDGE_LIST_WINDOWS: [&str; 3] = [
    "src/client/windows/autorun.rs",
    "src/client/windows/process.rs",
    "src/client/windows/charset.rs",
];

fn bridge_build() {
    #[cfg(feature = "windows")]
    BRIDGE_LIST_WINDOWS.iter().for_each(|x| {
        cxx_build::bridge(x);
    });
}

#[cfg(feature = "windows")]
fn xmake_build_windows(workspace: &Path) {
    XMAKE_PROJECT_LIST_WINDOWS
        .iter()
        .for_each(|project| xmake::build(workspace.join("m0n1t0r-client").join(project).as_path()));
}

fn xmake_build(workspace: &Path) {
    dep::check_xmake();
    dep::check_xrepo();

    #[cfg(feature = "windows")]
    xmake_build_windows(workspace);
}

#[cfg(feature = "windows-uac")]
fn add_administrator_manifest_windows() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
    );
    res.compile().unwrap();
}

fn main() {
    let workspace = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let certs = cert::path();

    if !cert::check(&certs) {
        panic!(
            "No certificates under {} found. Please run `cargo xtask -c` to generate one.",
            certs.display()
        );
    }

    bridge_build();
    xmake_build(workspace);

    #[cfg(feature = "windows-uac")]
    add_administrator_manifest_windows();
}
