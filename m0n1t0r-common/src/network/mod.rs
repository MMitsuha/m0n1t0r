use crate::{util, Result as AppResult};
use remoc::rtc;
use std::path::PathBuf;
use url::Url;

#[rtc::remote]
pub trait Agent: Sync {
    async fn download(&self, url: Url, path: PathBuf) -> AppResult<()> {
        util::network::download(url, &path).await
    }
}
