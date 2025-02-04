use anyhow::{anyhow, Result};
use clap::Parser;
use flexi_logger::Logger;
use log::{error, info};
use std::{env, process::Command};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    upx: bool,
}

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let args = Arguments::parse();

    if args.upx == true {
        info!("Using UPX located at: {}", env!("UPX_EXECUTABLE"));

        let current = env::current_exe()?;
        let pwd = current
            .parent()
            .ok_or(anyhow!("Failed to get current executable path"))?;
        let client = pwd.join("m0n1t0r-client");

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

    Ok(())
}
