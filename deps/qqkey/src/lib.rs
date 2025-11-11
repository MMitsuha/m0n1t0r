mod account;
mod error;
mod group;
mod qq;
mod qzone;

pub use account::Account;
pub use error::*;
pub use group::{Group, Role as GroupRole};
pub use qq::{Info as AccountInfo, QQ};
pub use qzone::{Group as FriendGroup, QZone};

const LOGIN_REFERER: &str = r"https://xui.ptlogin2.qq.com/";
const QQ_REFERER: &str = r"https://ptlogin2.qq.com/";
