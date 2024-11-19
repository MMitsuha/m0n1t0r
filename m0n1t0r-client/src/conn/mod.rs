pub type ClientMap = HashMap<SocketAddr, Arc<RwLock<ClientObj>>>;

use crate::ClientObj;
use anyhow::{anyhow, bail, Result};
use log::{debug, info, warn};
use m0n1t0r_common::{
    client::{ClientClient, ClientServerSharedMut},
    server::ServerClient,
};
use remoc::{
    prelude::ServerSharedMut,
    rch::{
        self,
        base::{Receiver, Sender},
    },
    Cfg, Connect,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    io,
    net::{self, TcpStream},
    select,
    sync::RwLock,
    time,
};
use tokio_util::sync::CancellationToken;

pub struct Config {
    addr: String,
    client_map: Arc<RwLock<ClientMap>>,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
        Self {
            addr: config.addr.clone(),
            client_map: config.client_map.clone(),
        }
    }
}

/// Connect to a client and create a channel for client exchange.
async fn make_channel<'transport>(
    canceller: CancellationToken,
    addr: &SocketAddr,
    stream: TcpStream,
) -> Result<(Sender<ClientClient>, Receiver<ServerClient>)> {
    let addr = addr.clone();
    let (socket_rx, socket_tx) = io::split(stream);
    let (conn, tx, rx): (
        _,
        rch::base::Sender<ClientClient>,
        rch::base::Receiver<ServerClient>,
    ) = Connect::io(Cfg::default(), socket_rx, socket_tx).await?;

    tokio::spawn(async move {
        select! {
            _ = conn => canceller.cancel(),
            _ = canceller.cancelled() => {},
        };

        debug!("{}: connection closed", addr);
    });
    Ok((tx, rx))
}

async fn server_task(
    canceller: CancellationToken,
    addr: SocketAddr,
    client_server: ClientServerSharedMut<ClientObj>,
    client_map: Arc<RwLock<ClientMap>>,
) {
    select! {
        _ = client_server.serve(true) => canceller.cancel(),
        _ = canceller.cancelled() => {},
    }

    match client_map.write().await.remove(&addr) {
        Some(_server) => {
            info!("{}: disconnected", addr);
        }
        None => {
            warn!("{}: disconnected unexpectedly", addr);
        }
    }
}

pub async fn accept(addr: &SocketAddr, client_map: Arc<RwLock<ClientMap>>) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let addr = addr.clone();
    debug!("{}: connection opened", addr);
    let client = Arc::new(RwLock::new(ClientObj::new(&addr)));
    let canceller = client.read().await.get_canceller();
    let guard = canceller.clone().drop_guard();
    let terminator = client.read().await.get_terminator();
    let (mut tx, mut rx) = make_channel(canceller.clone(), &addr, stream).await?;
    let (client_server, client_client) = ClientServerSharedMut::<_>::new(client.clone(), 1);

    let server_client = rx.recv().await?.ok_or(anyhow!("server is invalid"))?;
    tx.send(client_client).await?;

    tokio::spawn(server_task(
        canceller.clone(),
        addr,
        client_server,
        client_map.clone(),
    ));
    client.write().await.initialize(server_client);
    client_map.write().await.insert(addr, client);
    guard.disarm();
    info!("{}: connected", addr);

    select! {
        _ = canceller.cancelled() => {
            bail!("connection lost")
        },
        _ = terminator.cancelled() => {
            canceller.cancel();
            Ok(())
        },
    }
}

pub async fn run(config: &Config) -> Result<()> {
    while let Err(e) = accept(
        &loop {
            match net::lookup_host(&config.addr)
                .await?
                .next()
                .ok_or(anyhow!("no address found"))
            {
                Ok(a) => break a,
                Err(_) => continue,
            }
        },
        config.client_map.clone(),
    )
    .await
    {
        warn!("failed to connect server: {}", e);
        time::sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}
