use anyhow::Result;
use rsntp::AsyncSntpClient;
use std::time::Duration;
use tokio::time::Instant;

pub async fn ntp() -> Result<Duration> {
    let client = AsyncSntpClient::new();
    let result = client.synchronize("pool.ntp.org").await?;

    Ok(result.datetime().unix_timestamp()?)
}

pub async fn ping(before: Duration) -> Result<Duration> {
    let start = Instant::now();
    let now = ntp().await?;
    let end = Instant::now();
    let cost = now - (end - start) - before;

    Ok(cost)
}
