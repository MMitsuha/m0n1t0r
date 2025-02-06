#![allow(unused)]

use std::{
    path::{Path, PathBuf},
    process::Command,
};

const XMAKE_PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];
const BRIDGE_LIST_WINDOWS: [&str; 1] = ["src/client/windows/process.rs"];

fn bridge_build() {
    #[cfg(feature = "windows")]
    BRIDGE_LIST_WINDOWS.iter().for_each(|x| {
        cxx_build::bridge(x);
    });
}

fn check_certs(workspace: &Path) {
    let certs = workspace.join("certs");
    cargo_emit::rerun_if_changed!(certs.display());
}

fn check_xmake_dependencies() {
    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");
}

#[cfg(feature = "windows")]
fn xmake_build_windows(paths: &mut Vec<PathBuf>, workspace: &Path) {
    paths.append(
        &mut XMAKE_PROJECT_LIST_WINDOWS
            .iter()
            .map(|x| xmake::build(workspace.join("m0n1t0r-client").join(x).as_path()))
            .collect::<Vec<PathBuf>>(),
    );
    XMAKE_PROJECT_LIST_WINDOWS.iter().for_each(|x| {
        cargo_emit::rustc_link_lib!(x);
    });
}

fn xmake_build(workspace: &Path) {
    check_xmake_dependencies();

    let mut paths: Vec<PathBuf> = Vec::new();

    #[cfg(feature = "windows")]
    xmake_build_windows(&mut paths, workspace);

    for path in paths {
        cargo_emit::rustc_link_search!(path.display());
    }
}

#[cfg(feature = "windows")]
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

    check_certs(workspace);

    bridge_build();
    xmake_build(workspace);

    #[cfg(feature = "windows")]
    add_administrator_manifest_windows();
}
