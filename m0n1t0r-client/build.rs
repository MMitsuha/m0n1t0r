#![allow(unused)]

#[cfg(not(any(
    feature = "winnt",
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
    #[cfg(feature = "winnt")]
    BRIDGE_LIST_WINDOWS.iter().for_each(|x| {
        cxx_build::bridge(x);
    });
}

#[cfg(feature = "winnt")]
fn xmake_build_windows() {
    XMAKE_PROJECT_LIST_WINDOWS.iter().for_each(|project| {
        xmake::build(
            Path::new(env!("CARGO_WORKSPACE_DIR"))
                .join("m0n1t0r-client")
                .join(project)
                .as_path(),
        )
    });
}

fn xmake_build() {
    dep::check_xmake();
    dep::check_xrepo();

    #[cfg(feature = "winnt")]
    xmake_build_windows();
}

#[cfg(feature = "winnt")]
fn add_manifest_windows() {
    let mut res = winres::WindowsResource::new();
    res.set_icon(
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join("resource/mc.ico")
            .to_str()
            .unwrap(),
    )
    .set_language(winapi::um::winnt::MAKELANGID(
        winapi::um::winnt::LANG_ENGLISH,
        winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    #[cfg(feature = "winnt-uac")]
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
    let certs = cert::path();

    if !cert::check(&certs) {
        panic!(
            "No certificates under {} found. Please run `cargo xtask -c` to generate one.",
            certs.display()
        );
    }

    bridge_build();
    xmake_build();

    #[cfg(feature = "winnt")]
    add_manifest_windows();
}
