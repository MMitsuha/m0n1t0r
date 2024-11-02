use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_server::Config;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let config = Config::new();

    m0n1t0r_server::run(&config).await?;
    Ok(())
}
