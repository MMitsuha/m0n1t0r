use crate::Result as AppResult;
use qqkey::{Account, QQ};
use std::sync::Arc;

pub async fn account_by_id(id: i64) -> AppResult<Option<Account>> {
    let qq = Arc::new(QQ::new().await?);

    for info in qq.logged_qq().await? {
        if info.uin != id {
            continue;
        }

        return Ok(Some(Account::new(qq.clone(), id).await?));
    }

    return Ok(None);
}
