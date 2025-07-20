use anyhow::{anyhow, Result};
use clap::Parser;
use flexi_logger::Logger;
use log::{error, info, warn};
use m0n1t0r_build::cert;
use std::{env, io, process::Command};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    upx: bool,
    #[arg(short, long)]
    cert: bool,
}

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let args = Arguments::parse();

    if args.upx {
        info!("Using UPX located at: {}", env!("UPX_EXECUTABLE"));

        let current = env::current_exe()?;
        let pwd = current
            .parent()
            .ok_or(anyhow!("Failed to get current executable path"))?;
        #[allow(unused_mut)]
        let mut client = pwd.join("m0n1t0r-client");

        #[cfg(windows)]
        client.set_extension("exe");

        info!("Compressing binary({}) with UPX", client.display());

        let status = Command::new(env!("UPX_EXECUTABLE"))
            .arg("-9")
            .arg(&client)
            .status()?;

        match status.success() {
            true => info!("Successfully compressed"),
            false => error!("Failed to compress"),
        }
    }

    if args.cert {
        let certs = cert::path();

        if cert::check_no_rerun(&certs) {
            let mut input = String::new();
            warn!(
                "Certificates found under {}. Should continue(y/N)?",
                certs.display()
            );
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                return Ok(());
            }
        }

        cert::generate(&certs);
    }

    Ok(())
}
