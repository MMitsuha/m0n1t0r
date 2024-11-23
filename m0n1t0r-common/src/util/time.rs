use anyhow::Result;
use rsntp::AsyncSntpClient;
use std::time::Duration;

pub async fn ntp() -> Result<Duration> {
    let client = AsyncSntpClient::new();
    let result = client.synchronize("pool.ntp.org").await?;

    Ok(result.datetime().unix_timestamp()?)
}
