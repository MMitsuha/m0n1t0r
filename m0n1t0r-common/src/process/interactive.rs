use crate::Result as AppResult;
use anyhow::Error;
use remoc::{
    chmux::ReceiverStream,
    rch::bin::{self, Receiver, Sender},
};
use std::process::Stdio;
use tokio::{process::Command, select};
use tokio_util::io::{CopyToBytes, SinkWriter, StreamReader};

pub async fn interactive(command: String) -> AppResult<(Sender, Receiver, Receiver)> {
    let (stdin_tx, stdin_rx) = bin::channel();
    let (stdout_tx, stdout_rx) = bin::channel();
    let (stderr_tx, stderr_rx) = bin::channel();
    let mut process = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(Error::from)?;

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
            _ = tokio::io::copy(
                &mut stdin_rx,
                &mut stdin,
            ) => process.kill().await?,
            _ = tokio::io::copy(
                &mut stdout,
                &mut stdout_sink,
            ) => process.kill().await?,
            _ = tokio::io::copy(
                &mut stderr,
                &mut stderr_sink,
            ) => process.kill().await?,
            _ = process.wait() => {},
        }

        Ok::<_, Error>(())
    });
    Ok((stdin_tx, stdout_rx, stderr_rx))
}
