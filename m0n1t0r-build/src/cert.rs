use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use crate::dep;

const CA_CERT: &str = "ca.crt";
const END_KEY: &str = "end.key";
const END_CERT: &str = "end.crt";
const CERT_EXT: &str = "cert.ext";

pub fn path() -> PathBuf {
    Path::new(env!("CARGO_WORKSPACE_DIR")).join("certs")
}

pub fn check(certs: &Path) -> bool {
    cargo_emit::rerun_if_changed!(certs.display());
    check_no_rerun(certs)
}

pub fn check_no_rerun(certs: &Path) -> bool {
    [
        certs.join(CA_CERT),
        certs.join(END_KEY),
        certs.join(END_CERT),
    ]
    .into_iter()
    .any(|p| p.exists() == false)
        == false
}

pub fn generate(certs: &Path) {
    dep::check_openssl();

    // TODO: Fix this hack
    let ca_key = certs.join("ca.key");
    let end_csr = certs.join("end.csr");
    let cert_ext = certs.join(CERT_EXT);
    let ca_crt = certs.join(CA_CERT);
    let end_key = certs.join(END_KEY);
    let end_crt = certs.join(END_CERT);

    let ca_key = ca_key.to_str().unwrap();
    let end_csr = end_csr.to_str().unwrap();
    let cert_ext = cert_ext.to_str().unwrap();
    let ca_crt = ca_crt.to_str().unwrap();
    let end_key = end_key.to_str().unwrap();
    let end_crt = end_crt.to_str().unwrap();

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
            ca_key,
            "-out",
            ca_crt,
            "-nodes",
            "-subj",
            concat!(
                // TODO: Customize this
                "/C=CN/ST=ShangHai/L=ShangHai/O=",
                env!("M0N1T0R_ORG"),
                "/OU=./CN=",
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
            end_key,
            "-pkeyopt",
            "rsa_keygen_bits:4096",
        ],
        vec![
            "openssl",
            "req",
            "-new",
            "-key",
            end_key,
            "-out",
            end_csr,
            "-subj",
            concat!(
                // TODO: Customize this
                "/C=CN/ST=ShangHai/L=ShangHai/O=",
                env!("M0N1T0R_ORG"),
                "/OU=./CN=",
                env!("M0N1T0R_DOMAIN"),
                "."
            ),
        ],
        vec![
            "openssl",
            "x509",
            "-req",
            "-in",
            end_csr,
            "-CA",
            ca_crt,
            "-CAkey",
            ca_key,
            "-CAcreateserial",
            "-extfile",
            cert_ext,
            "-out",
            end_crt,
            "-days",
            "3650",
            "-sha256",
        ],
    ];

    fs::create_dir_all(certs).expect("Failed to create certs directory.");
    File::create(certs.join(CERT_EXT))
        .expect("Failed to create cert.ext file.")
        .write(
            concat!(
                "basicConstraints=CA:FALSE\nsubjectAltName = @alt_names\n[alt_names]\nDNS.1 = ",
                env!("M0N1T0R_DOMAIN"),
                "\n"
            )
            .as_bytes(),
        )
        .expect("Failed to write to cert.ext file.");

    if commands
        .into_iter()
        .map(|c| {
            Command::new(c[0])
                .args(&c[1..])
                .spawn()
                .expect("Failed to execute openssl.")
                .wait()
        })
        .any(|r| r.unwrap().success() == false)
        == true
    {
        panic!("Failed to generate certificates");
    }
}
