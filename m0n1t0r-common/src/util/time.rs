use anyhow::Result;
use chrono::{DateTime, Local};
use rsntp::AsyncSntpClient;
use std::time::{Duration, SystemTime};

pub async fn ntp() -> Result<Duration> {
    let client = AsyncSntpClient::new();
    let result = client.synchronize("pool.ntp.org").await?;

    Ok(result.datetime().unix_timestamp()?)
}

pub fn system() -> Result<Duration> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?)
}

pub fn local() -> DateTime<Local> {
    Local::now()
}
