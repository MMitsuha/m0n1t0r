#![allow(unused)]

use m0n1t0r_build::{cert, dep};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const XMAKE_PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];
const BRIDGE_LIST_WINDOWS: [&str; 2] = [
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
    dep::check_xmake();

    let mut paths: Vec<PathBuf> = Vec::new();

    #[cfg(feature = "windows")]
    xmake_build_windows(&mut paths, workspace);

    for path in paths {
        let path = path.display();
        cargo_emit::rustc_link_search!(path);
        cargo_emit::rerun_if_changed!(path);
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
    let certs = cert::path();

    if cert::check(&certs) == false {
        panic!(
            "No certificates under {} found. Please run `cargo xtask -c` to generate one.",
            certs.display()
        );
    }

    bridge_build();
    xmake_build(workspace);

    #[cfg(feature = "windows")]
    add_administrator_manifest_windows();
}
