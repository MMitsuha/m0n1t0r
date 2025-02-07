use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_server::{Config, ServerMap};
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let path = Path::new(env!("CARGO_WORKSPACE_DIR")).join("certs");
    let config = Config::new(
        &"0.0.0.0:27853".parse()?,
        &"0.0.0.0:10801".parse()?,
        &path.join("end.key"),
        &path.join("end.crt"),
    )?;
    let server_map = Arc::new(RwLock::new(ServerMap::new()));

    m0n1t0r_server::run(&config, server_map).await?;
    Ok(())
}
