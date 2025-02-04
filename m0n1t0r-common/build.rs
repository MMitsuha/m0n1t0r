use std::{fs::File, io::Write, path::Path, process::Command};
use vergen::{BuildBuilder, CargoBuilder, Emitter, RustcBuilder, SysinfoBuilder};

fn check_certs(certs: &Path) -> bool {
    cargo_emit::rerun_if_changed!(certs.display());

    [
        certs.join("ca.crt"),
        certs.join("end.key"),
        certs.join("end.crt"),
    ]
    .into_iter()
    .any(|p| p.exists() == false)
}

fn generate_certs(certs: &Path) {
    let ca_key = certs.join("ca.key");
    let ca_crt = certs.join("ca.crt");
    let end_key = certs.join("end.key");
    let end_crt = certs.join("end.crt");
    let end_csr = certs.join("end.csr");
    let cert_ext = certs.join("cert.ext");
    let commands = vec![
        vec![
            "openssl",
            "req",
            "-x509",
            "-sha512",
            "-days",
            "3650",
            "-newkey",
            "rsa:4096",
            "-keyout",
            ca_key.to_str().unwrap(),
            "-out",
            ca_crt.to_str().unwrap(),
            "-nodes",
            "-subj",
            concat!(
                "/C=CN/ST=ShangHai/L=ShangHai/O=K and A Ltd/OU=./CN=",
                env!("M0N1T0R_DOMAIN"),
                "."
            ),
        ],
        vec![
            "openssl",
            "genpkey",
            "-algorithm",
            "RSA",
            "-out",
            end_key.to_str().unwrap(),
            "-pkeyopt",
            "rsa_keygen_bits:4096",
        ],
        vec![
            "openssl",
            "req",
            "-new",
            "-key",
            end_key.to_str().unwrap(),
            "-out",
            end_csr.to_str().unwrap(),
            "-subj",
            concat!(
                "/C=CN/ST=ShangHai/L=ShangHai/O=K and A Ltd/OU=./CN=",
                env!("M0N1T0R_DOMAIN"),
                "."
            ),
        ],
        vec![
            "openssl",
            "x509",
            "-req",
            "-in",
            end_csr.to_str().unwrap(),
            "-CA",
            ca_crt.to_str().unwrap(),
            "-CAkey",
            ca_key.to_str().unwrap(),
            "-CAcreateserial",
            "-extfile",
            cert_ext.to_str().unwrap(),
            "-out",
            end_crt.to_str().unwrap(),
            "-days",
            "3650",
            "-sha256",
        ],
    ];

    File::create(certs.join("cert.ext"))
        .unwrap()
        .write(
            concat!(
                "basicConstraints=CA:FALSE\nsubjectAltName = @alt_names\n[alt_names]\nDNS.1 = ",
                env!("M0N1T0R_DOMAIN"),
                "\n"
            )
            .as_bytes(),
        )
        .unwrap();

    commands.into_iter().for_each(|c| {
        Command::new(c[0]).args(&c[1..]);
    });
}

fn generate_version() {
    let build = BuildBuilder::all_build().unwrap();
    let cargo = CargoBuilder::all_cargo().unwrap();
    let rustc = RustcBuilder::all_rustc().unwrap();
    let si = SysinfoBuilder::all_sysinfo().unwrap();

    Emitter::default()
        .add_instructions(&build)
        .unwrap()
        .add_instructions(&cargo)
        .unwrap()
        .add_instructions(&rustc)
        .unwrap()
        .add_instructions(&si)
        .unwrap()
        .emit()
        .unwrap();
}

fn main() {
    let certs = Path::new(env!("CARGO_WORKSPACE_DIR")).join("certs");

    generate_version();

    if check_certs(&certs) == false {
        generate_certs(&certs);
    }
}
