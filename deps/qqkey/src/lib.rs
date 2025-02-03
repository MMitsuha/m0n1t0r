mod account;
mod error;
mod group;
mod qq;
mod qzone;

pub use account::{
    Account, AccountList, Info as AccountInfo, InfoList as AccountInfoList, UrlList,
};
pub use error::*;
pub use group::{Group, GroupList, Role as GroupRole};
pub use qq::QQ;
pub use qzone::QZone;

const LOGIN_REFERER: &'static str = r"https://xui.ptlogin2.qq.com/";
const QQ_REFERER: &'static str = r"https://ptlogin2.qq.com/";
