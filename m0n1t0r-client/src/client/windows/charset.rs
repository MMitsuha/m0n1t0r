use m0n1t0r_common::Result as AppResult;
use std::thread;
use tokio::sync::oneshot;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

impl m0n1t0r_common::charset::Agent for AgentObj {
    async fn acp_to_utf8(&self, string: Vec<u8>) -> AppResult<String> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(ffi::acp_to_utf8(string));
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await??)
    }

    async fn acp(&self) -> AppResult<u32> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(ffi::acp());
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await??)
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {}

    unsafe extern "C++" {
        include!("m0n1t0r-client/m0n1t0r-cpp-windows-lib/include/charset.h");

        fn acp_to_utf8(string: Vec<u8>) -> Result<String>;

        fn acp() -> Result<u32>;
    }
}
