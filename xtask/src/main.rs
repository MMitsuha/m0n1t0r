use anyhow::Result;
use clap::Parser;
use flexi_logger::Logger;
use log::warn;
use m0n1t0r_build::cert;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    cert: bool,
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

        cert::generate(&certs);
    }

    Ok(())
}
