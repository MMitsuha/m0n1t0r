use crate::Result as AppResult;
use qqkey::{Account, AccountInfo, AccountInfoList, UrlList, QQ};
use remoc::rtc;

pub async fn get_account(id: i64) -> AppResult<Option<Account>> {
    let qq = QQ::new().await?;

    for i in QQ::new().await?.get_logged_qq_info().await? {
        // TODO: Figure which field should be used
        if i.account != id {
            continue;
        }

        return Ok(Some(Account::from(&qq, i).await?));
    }

    return Ok(None);
}
