use anyhow::Result;
use flexi_logger::Logger;
use m0n1t0r_server::Config;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let config = Config::new(&"0.0.0.0:27853".parse()?, &"0.0.0.0:8080".parse()?);

    m0n1t0r_server::run(&config).await?;
    Ok(())
}
