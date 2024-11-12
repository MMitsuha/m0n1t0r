use cfg_block::cfg_block;

cfg_block! {
    #[cfg(feature = "general")] {
        mod general;
        pub use general::*;
    }

    #[cfg(feature = "windows")] {
        mod windows;
        pub use windows::*;
    }
}
