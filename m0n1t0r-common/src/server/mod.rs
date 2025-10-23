use crate::{Result as AppResult, util};
use remoc::rtc;

#[rtc::remote]
pub trait Server: Sync {
    async fn version(&self) -> AppResult<String> {
        Ok(util::version::version().into())
    }

    async fn build_time(&self) -> AppResult<String> {
        Ok(util::version::build_time().into())
    }

    async fn commit_hash(&self) -> AppResult<String> {
        Ok(util::version::commit_hash().into())
    }

    async fn ping(&self) -> AppResult<()> {
        Ok(())
    }
}
