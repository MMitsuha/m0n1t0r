mod client;
mod conn;

use anyhow::Result;

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn run(config: &Config) -> Result<()> {
    let conn_config = conn::Config::new(&"127.0.0.1:27853".parse()?);

    conn::run(&conn_config).await?;
    Ok(())
}
