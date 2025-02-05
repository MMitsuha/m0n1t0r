mod util;

use crate::{Error, Result as AppResult};
use qqkey::{AccountInfoList, FriendGroup, UrlList, QQ};
use remoc::rtc;
use std::collections::HashMap;

#[rtc::remote]
pub trait Agent: Sync {
    async fn list(&self) -> AppResult<AccountInfoList> {
        Ok(QQ::new().await?.get_logged_qq_info().await?)
    }

    async fn urls(&self, id: i64) -> AppResult<UrlList> {
        let account = util::get_account(id).await?.ok_or(Error::NotFound)?;
        Ok(UrlList::new(&account).await?)
    }

    async fn friends(&self, id: i64) -> AppResult<HashMap<i64, FriendGroup>> {
        let account = util::get_account(id).await?.ok_or(Error::NotFound)?;
        let qzone = account.get_qzone().await?;

        Ok(qzone.get_friends().await?)
    }
}
