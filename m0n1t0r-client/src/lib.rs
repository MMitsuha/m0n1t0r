mod client;
mod conn;

use std::net::SocketAddr;

pub use client::ClientObj;

use anyhow::Result;

pub struct Config {
    addr: SocketAddr,
}

impl Config {
    pub fn new(addr: &SocketAddr) -> Self {
        Self { addr: addr.clone() }
    }
}

pub async fn run(config: &Config) -> Result<()> {
    conn::run(&config.into()).await?;
    Ok(())
}
