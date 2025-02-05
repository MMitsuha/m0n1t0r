use crate::Result as AppResult;
use std::path::PathBuf;
use tokio::fs::{self};
use url::Url;

pub async fn download(url: Url, path: &PathBuf) -> AppResult<()> {
    let response = reqwest::get(url).await?;
    fs::write(path, response.bytes().await?).await?;
    Ok(())
}
