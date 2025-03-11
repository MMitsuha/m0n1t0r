use crate::Result as AppResult;
use qqkey::{Account, QQ};
use std::sync::Arc;

pub async fn get_account(id: i64) -> AppResult<Option<Account>> {
    let qq = Arc::new(QQ::new().await?);

    for info in qq.get_logged_qq().await? {
        if info.uin != id {
            continue;
        }

        return Ok(Some(Account::new(qq.clone(), id).await?));
    }

    return Ok(None);
}
