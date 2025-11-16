use m0n1t0r_common::{Error, Result as AppResult};
use std::thread;
use tokio::sync::oneshot;

pub async fn init() -> AppResult<bool> {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let _ = tx.send(ffi::init()?);
        Ok::<_, Error>(())
    });
    Ok(rx.await?)
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {}

    unsafe extern "C++" {
        include!("m0n1t0r-client/m0n1t0r-cpp-general-lib/include/init.h");

        fn init() -> Result<bool>;
    }
}
