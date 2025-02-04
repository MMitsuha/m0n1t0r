use crate::Result as AppResult;
use serde::{Deserialize, Serialize};
use std::process::{self};
use tokio::process::Command;

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

pub async fn execute(command: String, args: Vec<String>) -> AppResult<Output> {
    Ok(Command::new(command).args(args).output().await?.into())
}

pub fn execute_detached(command: String, args: Vec<String>) -> AppResult<()> {
    Command::new(command).args(args).spawn()?;
    Ok(())
}
