use crate::{Error, Result as AppResult};
use remoc::{
    chmux::ReceiverStream,
    rch::bin::{self, Receiver, Sender},
    rtc,
};
use std::net::SocketAddr;
use tokio::{io, net::TcpStream, select};
use tokio_util::io::{CopyToBytes, SinkWriter, StreamReader};

#[rtc::remote]
pub trait Agent: Sync {
    async fn connect(&self, addr: SocketAddr) -> AppResult<(Sender, Receiver)> {
        let (mut stream_rx, mut stream_tx) = TcpStream::connect(addr).await?.into_split();
        let (tx, remote_rx) = bin::channel();
        let (remote_tx, rx) = bin::channel();

        tokio::spawn(async move {
            let mut rx = StreamReader::new(ReceiverStream::new(rx.into_inner().await?));
            let mut tx = SinkWriter::new(CopyToBytes::new(tx.into_inner().await?.into_sink()));

            select! {
                _ = io::copy(&mut rx, &mut stream_tx) => {},
                _ = io::copy(&mut stream_rx, &mut tx) => {},
            }
            Ok::<_, Error>(())
        });

        Ok((remote_tx, remote_rx))
    }
}
