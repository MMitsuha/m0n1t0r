use crate::Result as AppResult;
use anyhow::Error;
use remoc::rtc;
use serde::{Deserialize, Serialize};
use std::process;
use tokio::process::Command;

#[rtc::remote]
pub trait Agent: Sync {
    async fn execute(&self, command: String, args: Vec<String>) -> AppResult<Output> {
        Ok(Command::new(command)
            .args(args)
            .output()
            .await
            .map_err(Error::from)?
            .into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl From<process::Output> for Output {
    fn from(value: process::Output) -> Self {
        Self {
            success: value.status.success(),
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}
