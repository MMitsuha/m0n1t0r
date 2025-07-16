use crate::{Error, Result as AppResult};
use remoc::{
    chmux::ReceiverStream,
    rch::{bin, lr},
    rtc,
};
use std::net::SocketAddr;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    select,
};
use tokio_util::io::{CopyToBytes, SinkWriter, StreamReader};

#[rtc::remote]
pub trait Agent: Sync {
    async fn connect(&self, addr: SocketAddr) -> AppResult<(bin::Sender, bin::Receiver)> {
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

    async fn forward(
        &self,
        addr: SocketAddr,
    ) -> AppResult<(
        lr::Receiver<(bin::Sender, bin::Receiver, SocketAddr)>,
        lr::Sender<()>,
    )> {
        let listener = TcpListener::bind(addr).await?;
        let (mut self_tx, remote_rx) = lr::channel();
        let (canceller_tx, mut canceller_rx) = lr::channel();

        tokio::spawn(async move {
            loop {
                select! {
                    _ = canceller_rx.recv() => break,
                    accepted = listener.accept() => {
                        let (stream, addr) = accepted?;
                        let (mut stream_rx, mut stream_tx) = stream.into_split();
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

                        self_tx.send((remote_tx, remote_rx, addr)).await?;
                    },
                }
            }
            Ok::<_, Error>(())
        });

        Ok((remote_rx, canceller_tx))
    }
}
