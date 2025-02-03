mod util;

use crate::{Error, Result as AppResult};
use qqkey::{Account, AccountInfo, AccountInfoList, UrlList, QQ};
use remoc::rtc;

#[rtc::remote]
pub trait Agent: Sync {
    async fn list(&self) -> AppResult<AccountInfoList> {
        Ok(QQ::new().await?.get_logged_qq_info().await?)
    }

    async fn urls(&self, id: i64) -> AppResult<UrlList> {
        let account = util::get_account(id).await?.ok_or(Error::NotFound)?;
        Ok(UrlList::new(&account).await?)
    }
}
