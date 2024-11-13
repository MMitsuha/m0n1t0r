use cfg_block::cfg_block;

cfg_block! {
    #[cfg(all(
        feature = "general",
        not(feature = "windows"),
        not(feature = "linux"),
        not(feature = "macos"),
    ))] {
        mod general;
        pub use general::*;
    }

    #[cfg(feature = "windows")] {
        mod windows;
        pub use windows::*;
    }
}
