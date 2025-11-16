#![allow(unused)]

#[cfg(not(any(
    feature = "winnt",
    feature = "linux",
    feature = "macos",
    feature = "general"
)))]
compile_error!("No target platform specified.");

use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(feature = "winnt")]
fn add_manifest_windows() {
    let mut res = winres::WindowsResource::new();
    res.set_icon(
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join("resource/ms.ico")
            .to_str()
            .unwrap(),
    )
    .set_language(winapi::um::winnt::MAKELANGID(
        winapi::um::winnt::LANG_ENGLISH,
        winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    res.compile().unwrap();
}

fn main() {
    #[cfg(feature = "winnt")]
    add_manifest_windows();
}
