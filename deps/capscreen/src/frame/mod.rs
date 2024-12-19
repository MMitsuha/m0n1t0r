cfg_block! {
    #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::*;
    }
    #[cfg(target_os = "windows")] {
        pub(crate) mod windows;
        pub use windows::*;
    }
    #[cfg(target_os = "linux")] {
        pub(crate) mod linux;
        pub use linux::*;
    }
}

use cfg_block::cfg_block;

pub enum Frame {
    Nv12(Nv12),
    Bgra8(Bgra8),
    Empty,
}

pub struct Nv12 {
    pub width: u32,
    pub height: u32,
    pub y: Vec<u8>,
    pub y_stride: usize,
    pub uv: Vec<u8>,
    pub uv_stride: usize,
}

pub struct Bgra8 {
    pub width: u32,
    pub row_stride: u32,
    pub height: u32,
    pub height_stride: u32,
    pub data: Vec<u8>,
}
