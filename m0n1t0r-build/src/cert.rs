use std::{fs::File, io::Write, path::Path, process::Command};

const CA_CERT: &str = "ca.crt";
const END_KEY: &str = "end.key";
const END_CERT: &str = "end.crt";

pub fn check(certs: &Path) -> bool {
    cargo_emit::rerun_if_changed!(certs.display());

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
    let ca_key = certs.join("ca.key");
    let end_csr = certs.join("end.csr");
    let cert_ext = certs.join("cert.ext");
    let ca_crt = certs.join(CA_CERT);
    let end_key = certs.join(END_KEY);
    let end_crt = certs.join(END_CERT);
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
