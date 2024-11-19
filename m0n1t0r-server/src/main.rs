use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_server::Config;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let config = Config::new(&"0.0.0.0:27853".parse()?, &"0.0.0.0:10801".parse()?);
    let server_map = Arc::new(RwLock::new(HashMap::new()));

    m0n1t0r_server::run(&config, server_map).await?;
    Ok(())
}
