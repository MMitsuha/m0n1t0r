mod util;

use crate::{Error, Result as AppResult};
use qqkey::{Account, AccountInfo, FriendGroup, QQ};
use remoc::rtc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlList {
    qzone: String,
    weiyun: String,
    mail: String,
    qun: String,
}

impl UrlList {
    async fn new(account: &Account) -> AppResult<Self> {
        Ok(Self {
            qzone: account.get_qzone_url(),
            weiyun: account.get_weiyun_url(),
            mail: account.get_mail_url(),
            qun: account.get_qun_url().await?,
        })
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn list(&self) -> AppResult<Vec<AccountInfo>> {
        Ok(QQ::new().await?.get_logged_qq().await?)
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
