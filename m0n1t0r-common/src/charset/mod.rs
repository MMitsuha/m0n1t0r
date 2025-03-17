use crate::{Error, Result as AppResult};
use remoc::rtc;

#[rtc::remote]
pub trait Agent: Sync {
    async fn acp_to_utf8(&self, _string: Vec<u8>) -> AppResult<String> {
        Err(Error::Unsupported)
    }

    async fn acp(&self) -> AppResult<u32> {
        Err(Error::Unsupported)
    }
}
