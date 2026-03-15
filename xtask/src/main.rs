use anyhow::{Context, Result};
use clap::Parser;
use flexi_logger::Logger;
use log::{info, warn};
use m0n1t0r_build::cert;
use m0n1t0r_common::config::{CertConfig, FileConfig};
use rcgen::{CertificateParams, DnType, IsCa, KeyPair};
use std::fs;
use time::{Duration, OffsetDateTime};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    cert: bool,
}

fn generate_certs(config: &CertConfig, certs_dir: &std::path::Path) -> Result<()> {
    fs::create_dir_all(certs_dir)?;

    let now = OffsetDateTime::now_utc();
    let not_after = now + Duration::days(3650);

    // Generate CA
    let mut ca_params = CertificateParams::new(Vec::<String>::new())?;
    ca_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.not_before = now;
    ca_params.not_after = not_after;
    ca_params
        .distinguished_name
        .push(DnType::CountryName, &config.country);
    ca_params
        .distinguished_name
        .push(DnType::StateOrProvinceName, &config.state);
    ca_params
        .distinguished_name
        .push(DnType::LocalityName, &config.locality);
    ca_params
        .distinguished_name
        .push(DnType::OrganizationName, &config.org);
    ca_params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, &config.unit);
    ca_params
        .distinguished_name
        .push(DnType::CommonName, format!("{}.", &config.domain));

    let ca_key = KeyPair::generate()?;
    let ca_cert = ca_params.self_signed(&ca_key)?;

    // Generate end entity
    let mut end_params = CertificateParams::new(vec![config.domain.clone()])?;
    end_params.is_ca = IsCa::NoCa;
    end_params.not_before = now;
    end_params.not_after = not_after;
    end_params
        .distinguished_name
        .push(DnType::CountryName, &config.country);
    end_params
        .distinguished_name
        .push(DnType::StateOrProvinceName, &config.state);
    end_params
        .distinguished_name
        .push(DnType::LocalityName, &config.locality);
    end_params
        .distinguished_name
        .push(DnType::OrganizationName, &config.org);
    end_params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, &config.unit);
    end_params
        .distinguished_name
        .push(DnType::CommonName, format!("{}.", &config.domain));

    let end_key = KeyPair::generate()?;
    let end_cert = end_params.signed_by(&end_key, &ca_cert, &ca_key)?;

    // Write files
    fs::write(certs_dir.join("ca.crt"), ca_cert.pem())?;
    fs::write(certs_dir.join("end.key"), end_key.serialize_pem())?;
    fs::write(certs_dir.join("end.crt"), end_cert.pem())?;

    info!("Certificates generated under {}", certs_dir.display());
    Ok(())
}

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let args = Arguments::parse();

    if args.cert {
        let certs = cert::path();

        if cert::check_no_rerun(&certs) {
            warn!("Certificates found under {}.", certs.display());
            return Ok(());
        }

        let config_path = "config.toml";
        let content =
            fs::read_to_string(config_path).context(format!("failed to read {config_path}"))?;
        let file_config: FileConfig =
            toml::from_str(&content).context(format!("failed to parse {config_path}"))?;

        generate_certs(&file_config.cert, &certs)?;
    }

    Ok(())
}
