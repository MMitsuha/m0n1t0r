use crate::Result as AppResult;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Shell {
    Zsh,
    Bash,
    Unknown,
}

impl Shell {
    pub fn new() -> AppResult<Self> {
        let shell_env = env::var("SHELL")?.to_lowercase();

        if shell_env.contains("zsh") {
            Ok(Shell::Zsh)
        } else if shell_env.contains("bash") {
            Ok(Shell::Bash)
        } else {
            Ok(Shell::Unknown)
        }
    }

    pub fn rc_file(&self) -> &'static str {
        match self {
            Shell::Zsh => ".zshrc",
            Shell::Bash => ".bashrc",
            Shell::Unknown => ".profile",
        }
    }
}
