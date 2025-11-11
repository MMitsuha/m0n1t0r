use std::sync::Arc;

use anyhow::Result;
use flexi_logger::Logger;
use log::info;
use qqkey::{Account, GroupRole, QQ};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;

    let qq = Arc::new(QQ::new().await?);

    for info in qq.logged_qq().await? {
        let account = Account::new(qq.clone(), info.uin).await?;

        info!("当前登录的QQ号: {}", info.uin);
        info!(
            "当前登录的QQ昵称: {}",
            info.nickname.unwrap_or("unknown".into())
        );

        info!("查询Url信息: ");
        info!("QQ邮箱: {}", account.mail_url());
        info!("QQ空间: {}", account.qzone_url());
        info!("微云: {}", account.weiyun_url());
        info!("QQ群: {}", account.qun_url().await?);

        info!("查询群信息: ");
        let groups = account.group_list().await?;

        for group in groups {
            let role = match group.role() {
                GroupRole::Owner => "群主",
                GroupRole::Admin => "管理员",
                GroupRole::Member => "成员",
            };

            if group.is_admin() {
                let detail = group.detail().await?;

                info!(
                    "[{}] {}: {} ({}/{})",
                    role,
                    group.id(),
                    group.name(),
                    detail.count,
                    detail.max_count
                );
            } else {
                info!("[{}] {}: {}", role, group.id(), group.name());
            }
        }
        println!();

        let qzone = account.qzone().await?;
        let friends = qzone.friends().await?;
        let special_cares = qzone.special_cares().await?;

        info!("好友: ");
        for (_, group) in friends {
            info!("分类: {}: ", group.name);
            for friend in group.friends {
                info!("{}: {}({})", friend.uin, friend.name, friend.remark);
            }
            println!();
        }
        println!();

        info!("特别关心: ");
        for item in special_cares {
            info!("{}: {}({})", item.uin, item.name, item.score);
        }
        println!();
    }

    Ok(())
}
