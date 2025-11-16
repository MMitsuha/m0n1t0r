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

fn bridge_build(bridges: &[&str]) {
    bridges.iter().for_each(|x| {
        cxx_build::bridge(x);
    });
}

fn xmake_build(project: &str) {
    let path = Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("m0n1t0r-client")
        .join(project);
    xmake::build(&path);
    cargo_emit::rerun_if_changed!(path.display());
}

#[cfg(feature = "winnt")]
fn add_manifest_windows() {
    use winapi::um::winnt;
    use winres::VersionInfo;

    let mut res = winres::WindowsResource::new();
    res.set_icon(
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join("resource/mc.ico")
            .to_str()
            .unwrap(),
    )
    .set_language(winnt::MAKELANGID(
        winnt::LANG_ENGLISH,
        winnt::SUBLANG_ENGLISH_US,
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
    dep::check_xmake();
    dep::check_xrepo();

    let certs = cert::path();

    if !cert::check(&certs) {
        panic!(
            "No certificates under {} found. Please run `cargo xtask -c` to generate one.",
            certs.display()
        );
    }

    bridge_build(&["src/init.rs"]);
    #[cfg(feature = "winnt")]
    bridge_build(&[
        "src/client/windows/autorun.rs",
        "src/client/windows/process.rs",
        "src/client/windows/charset.rs",
    ]);

    xmake_build("m0n1t0r-cpp-general-lib");
    #[cfg(feature = "winnt")]
    xmake_build("m0n1t0r-cpp-windows-lib");

    #[cfg(feature = "winnt")]
    add_manifest_windows();
}
