use anyhow::Result;
use flexi_logger::Logger;
use log::info;
use qqkey::{GroupRole, QQ};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;

    let qq = QQ::new().await?;
    let accounts = qq.get_logged_qq().await?;

    for account in accounts {
        info!("当前登录的QQ号: {}", account.get_uin());
        info!(
            "当前登录的QQ昵称: {}",
            account.get_nickname().unwrap_or("unknown".into())
        );
        info!("QQ邮箱: {}", account.get_mail_url());
        info!("QQ空间: {}", account.get_qzone_url());
        info!("微云: {}", account.get_weiyun_url());
        info!("QQ群: {}", account.get_qun_url().await?);

        info!("查询群信息: ");
        let groups = account.get_group_list().await?;

        for group in groups {
            let role = match group.get_role() {
                GroupRole::Owner => "群主",
                GroupRole::Admin => "管理员",
                GroupRole::Member => "成员",
            };

            if group.is_admin() {
                let detail = group.get_detail().await?;

                info!(
                    "[{}] {}: {} ({}/{})",
                    role,
                    group.get_id(),
                    group.get_name(),
                    detail.count,
                    detail.max_count
                );
            } else {
                info!("[{}] {}: {}", role, group.get_id(), group.get_name());
            }
        }
        println!();

        let qzone = account.get_qzone().await?;
        let friends = qzone.get_friends().await?;
        let special_cares = qzone.get_special_cares().await?;

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
