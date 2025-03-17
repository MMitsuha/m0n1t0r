use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Shell {
    Zsh,
    Bash,
    Unknown,
}

impl Shell {
    pub fn new() -> Self {
        env::var("SHELL")
            .map(|shell_env| match shell_env.to_lowercase().as_str() {
                "bash" => Shell::Bash,
                "zsh" => Shell::Zsh,
                _ => Shell::Unknown,
            })
            .unwrap_or(Shell::Unknown)
    }

    pub fn rc_file(&self) -> &'static str {
        match self {
            Shell::Zsh => ".zshrc",
            Shell::Bash => ".bashrc",
            Shell::Unknown => ".profile",
        }
    }
}
