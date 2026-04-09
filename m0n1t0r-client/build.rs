use m0n1t0r_build::{cert, config, dep};
use std::path::Path;

fn bridge_build(bridges: &[&str]) {
    bridges.iter().for_each(|x| {
        let _ = cxx_build::bridge(x);
    });
}

fn xmake_build(project: &str) {
    let path = Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("m0n1t0r-client")
        .join(project);
    xmake::build(&path);
    cargo_emit::rerun_if_changed!(path.display());
}

fn main() {
    m0n1t0r_build::platform::ensure();
    dep::check_xmake();
    dep::check_xrepo();

    config::ensure();
    cert::ensure();

    let cfg = config::read();
    cargo_emit::rustc_env!("M0N1T0R_DOMAIN", "{}", cfg.cert.domain);
    cargo_emit::rustc_env!("M0N1T0R_CONN_PORT", "{}", cfg.conn.addr.port());
    cargo_emit::rustc_env!(
        "M0N1T0R_CA",
        "{}",
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join(&cfg.tls.ca)
            .display()
    );

    bridge_build(&["src/init.rs"]);
    #[cfg(feature = "winnt")]
    bridge_build(&[
        "src/client/windows/autorun.rs",
        "src/client/windows/process.rs",
        "src/client/windows/charset.rs",
        "src/client/windows/blind.rs",
    ]);

    xmake_build("m0n1t0r-cpp-general-lib");
    #[cfg(feature = "winnt")]
    xmake_build("m0n1t0r-cpp-windows-lib");

    #[cfg(feature = "winnt")]
    {
        let icon = Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/mc.ico");
        #[cfg(feature = "winnt-uac")]
        let manifest = Some(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
        #[cfg(not(feature = "winnt-uac"))]
        let manifest = None;
        m0n1t0r_build::winres::embed(&icon, manifest);
    }
}
