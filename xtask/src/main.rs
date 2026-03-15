use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Input};
use flexi_logger::Logger;
use log::{info, warn};
use m0n1t0r_build::{
    cert as build_cert, config as build_config,
    config::{ApiConfig, CertConfig, ConnConfig, FileConfig, GeneralConfig, TlsConfig},
};
use rcgen::{CertificateParams, DnType, IsCa, KeyPair};
use std::fs;
use time::{Duration, OffsetDateTime};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long, help = "Generate TLS certificates")]
    cert: bool,
    #[arg(short, long, help = "Interactive config.toml generator")]
    init: bool,
}

fn generate_certs() -> Result<()> {
    let config = build_config::read();
    for path in [&config.tls.ca, &config.tls.cert, &config.tls.key] {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
    }

    let now = OffsetDateTime::now_utc();
    let not_after = now + Duration::days(3650);

    // Generate CA
    let mut ca_params = CertificateParams::new(Vec::<String>::new())?;
    ca_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.not_before = now;
    ca_params.not_after = not_after;
    ca_params
        .distinguished_name
        .push(DnType::CountryName, &config.cert.country);
    ca_params
        .distinguished_name
        .push(DnType::StateOrProvinceName, &config.cert.state);
    ca_params
        .distinguished_name
        .push(DnType::LocalityName, &config.cert.locality);
    ca_params
        .distinguished_name
        .push(DnType::OrganizationName, &config.cert.org);
    ca_params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, &config.cert.unit);
    ca_params
        .distinguished_name
        .push(DnType::CommonName, format!("{}.", &config.cert.domain));

    let ca_key = KeyPair::generate()?;
    let ca_cert = ca_params.self_signed(&ca_key)?;

    // Generate end entity
    let mut end_params = CertificateParams::new(vec![config.cert.domain.clone()])?;
    end_params.is_ca = IsCa::NoCa;
    end_params.not_before = now;
    end_params.not_after = not_after;
    end_params
        .distinguished_name
        .push(DnType::CountryName, &config.cert.country);
    end_params
        .distinguished_name
        .push(DnType::StateOrProvinceName, &config.cert.state);
    end_params
        .distinguished_name
        .push(DnType::LocalityName, &config.cert.locality);
    end_params
        .distinguished_name
        .push(DnType::OrganizationName, &config.cert.org);
    end_params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, &config.cert.unit);
    end_params
        .distinguished_name
        .push(DnType::CommonName, format!("{}.", &config.cert.domain));

    let end_key = KeyPair::generate()?;
    let end_cert = end_params.signed_by(&end_key, &ca_cert, &ca_key)?;

    // Write files
    fs::write(config.tls.ca, ca_cert.pem())?;
    fs::write(config.tls.key, end_key.serialize_pem())?;
    fs::write(config.tls.cert, end_cert.pem())?;

    info!("Certificates generated");
    Ok(())
}

fn prompt(name: &str, default: &str) -> Result<String> {
    Ok(Input::new()
        .with_prompt(name)
        .default(default.into())
        .interact_text()?)
}

fn init_config() -> Result<()> {
    let config_path = build_config::path();

    println!("=== General ===");
    let log_level = prompt("Log level", "debug")?;
    let secret = prompt(
        "Secret (session cookie key)",
        &uuid::Uuid::new_v4().to_string(),
    )?;

    println!("\n=== Connection ===");
    let conn_addr = prompt("Client listener address", "0.0.0.0:27853")?;

    println!("\n=== API ===");
    let api_addr = prompt("API address", "0.0.0.0:10801")?;
    let use_https = Confirm::new()
        .with_prompt("Use HTTPS for API?")
        .default(false)
        .interact()?;

    println!("\n=== TLS ===");
    let tls_ca = prompt("CA cert path", "certs/ca.crt")?;
    let tls_key = prompt("TLS key path", "certs/end.key")?;
    let tls_cert = prompt("TLS cert path", "certs/end.crt")?;

    println!("\n=== Certificate Subject ===");
    let country = prompt("Country", "CN")?;
    let state = prompt("State", "ShangHai")?;
    let locality = prompt("Locality", "ShangHai")?;
    let org = prompt("Organization", "m0n1t0r")?;
    let unit = prompt("Unit", ".")?;
    let domain = prompt("Domain", "localhost")?;

    let config = FileConfig {
        general: GeneralConfig { log_level, secret },
        conn: ConnConfig {
            addr: conn_addr.parse().context("invalid conn address")?,
        },
        api: ApiConfig {
            addr: api_addr.parse().context("invalid API address")?,
            use_https,
        },
        tls: TlsConfig {
            key: tls_key.into(),
            cert: tls_cert.into(),
            ca: tls_ca.into(),
        },
        cert: CertConfig {
            country,
            state,
            locality,
            org,
            unit,
            domain,
        },
    };

    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, &content)?;
    info!("Config written to {}", config_path.display());

    Ok(())
}

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let args = Arguments::parse();

    if args.init {
        if build_config::check() {
            warn!("Config found.");
            return Ok(());
        }
        init_config()?;
    }

    if args.cert {
        if !build_cert::check() {
            warn!("Certificates found.");
            return Ok(());
        }
        generate_certs()?;
    }

    Ok(())
}
