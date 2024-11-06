use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_client::Config;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let config = Config::new("mmitsuha.xyz:27853");

    m0n1t0r_client::run(&config).await?;
    Ok(())
}
