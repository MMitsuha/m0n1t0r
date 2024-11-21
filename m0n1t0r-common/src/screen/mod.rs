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
    async fn connect(&self) -> AppResult<()> {
        Ok(())
    }
}
