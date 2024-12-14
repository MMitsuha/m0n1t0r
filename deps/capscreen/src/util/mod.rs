cfg_block! {
    #[cfg(target_os = "macos")] {
        pub(crate) mod macos;
        pub use macos::*;
    }
    #[cfg(target_os = "windows")] {
        pub(crate) mod windows;
        pub use windows::*;
    }
}

use crate::Result;
use cfg_block::cfg_block;

pub trait Display {
    fn list() -> Result<Vec<Self>>
    where
        Self: Sized;

    fn main() -> Result<Self>
    where
        Self: Sized;
}

pub trait Permission {
    fn has_permission() -> bool;
    fn request_permission() -> bool;
    fn is_supported() -> Result<bool>;
}
