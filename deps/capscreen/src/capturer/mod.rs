cfg_block! {
    #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::*;
    }
    #[cfg(target_os = "windows")] {
        pub(crate) mod windows;
        pub use windows::*;
    }
}

use crate::{frame::Frame, Result};
use cfg_block::cfg_block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    display: Option<usize>,
    pub buffer_size: usize,
}

impl Config {
    pub fn main(buffer_size: usize) -> Self {
        Self {
            display: None,
            buffer_size,
        }
    }
}

pub trait Engine {
    fn new(config: &Config) -> Result<Self>
    where
        Self: Sized;

    fn start(&mut self) -> Result<()>;

    fn stop(&mut self) -> Result<()>;

    fn get_frame(&mut self) -> Result<Frame>;
}
