use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[allow(warnings)]
const PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];
const BRIDGE_LIST_WINDOWS: [&str; 1] = ["src/client/windows/process.rs"];

fn check_dependencies() {
    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");
}

fn xmake_build() -> Vec<PathBuf> {
    check_dependencies();

    let workspace = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let certs = workspace.join("certs");
    cargo_emit::rerun_if_changed!(certs.display());
    #[allow(warnings)]
    let mut paths = Vec::new();

    #[cfg(feature = "windows")]
    paths.append(
        &mut PROJECT_LIST_WINDOWS
            .iter()
            .map(|x| xmake::build(workspace.join("m0n1t0r-client").join(x).as_path()))
            .collect::<Vec<PathBuf>>(),
    );

    paths
}

fn add_administrator_manifest() {
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
    BRIDGE_LIST_WINDOWS.iter().for_each(|x| {
        let _ = cxx_build::bridge(x);
    });

    for path in xmake_build() {
        cargo_emit::rustc_link_search!(path.display());
    }

    #[cfg(feature = "windows")]
    PROJECT_LIST_WINDOWS.iter().for_each(|x| {
        cargo_emit::rustc_link_lib!(x);
    });

    #[cfg(windows)]
    add_administrator_manifest();
}
