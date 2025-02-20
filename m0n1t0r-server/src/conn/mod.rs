#[cfg(debug_assertions)]
use crate::server;

use crate::ServerObj;
use anyhow::{anyhow, Result};
use log::{debug, info, warn};
use m0n1t0r_common::{
    client::ClientClient,
    server::{ServerClient, ServerServerSharedMut},
};
use remoc::{
    prelude::ServerSharedMut,
    rch::{
        self,
        base::{Receiver, Sender},
    },
    Cfg, Connect,
};
use rustls_pki_types::{pem::PemObject as _, CertificateDer, PrivateKeyDer};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    select,
    sync::{watch, RwLock},
};
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ConnectEventEnum {
    Connect = 0,
    Disconnect = 1,
    Invalid = 2,
}

impl Default for ConnectEventEnum {
    fn default() -> Self {
        Self::Invalid
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConnectEvent {
    event: ConnectEventEnum,
    addr: SocketAddr,
}

impl Default for ConnectEvent {
    fn default() -> Self {
        Self {
            event: ConnectEventEnum::default(),
            addr: ([0, 0, 0, 0], 0).into(),
        }
    }
}

pub struct ServerMap {
    pub map: HashMap<SocketAddr, Arc<RwLock<ServerObj>>>,
    notify_tx: watch::Sender<ConnectEvent>,
    pub notify_rx: watch::Receiver<ConnectEvent>,
}

impl ServerMap {
    pub fn new() -> Self {
        let (notify_tx, notify_rx) = watch::channel(ConnectEvent::default());
        Self {
            map: HashMap::new(),
            notify_tx,
            notify_rx,
        }
    }
}

pub struct Config {
    addr: SocketAddr,
    key: PathBuf,
    cert: PathBuf,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
        Self {
            addr: config.conn_addr,
            key: config.key.clone(),
            cert: config.cert.clone(),
        }
    }
}

/// Connect to a client and create a channel for client exchange.
async fn make_channel<'transport>(
    canceller: CancellationToken,
    addr: &SocketAddr,
    stream: TlsStream<TcpStream>,
) -> Result<(Sender<ServerClient>, Receiver<ClientClient>)> {
    let addr = addr.clone();
    let (stream_rx, stream_tx) = io::split(stream);
    let (conn, tx, rx): (
        _,
        rch::base::Sender<ServerClient>,
        rch::base::Receiver<ClientClient>,
    ) = Connect::io(Cfg::default(), stream_rx, stream_tx).await?;

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
    server_server: ServerServerSharedMut<ServerObj>,
    server_map: Arc<RwLock<ServerMap>>,
) -> Result<()> {
    select! {
        _ = server_server.serve(true) => canceller.cancel(),
        _ = canceller.cancelled() => {},
    };

    let mut lock_map = server_map.write().await;
    match lock_map.map.remove(&addr) {
        Some(_server) => {
            info!("{}: disconnected", addr);
        }
        None => {
            warn!("{}: disconnected unexpectedly", addr);
        }
    }
    lock_map.notify_tx.send(ConnectEvent {
        event: ConnectEventEnum::Disconnect,
        addr,
    })?;
    drop(lock_map);
    Ok(())
}

pub async fn accept(
    listener: &TcpListener,
    acceptor: TlsAcceptor,
    server_map: Arc<RwLock<ServerMap>>,
) -> Result<()> {
    let (stream, addr) = listener.accept().await?;
    let stream = acceptor.accept(stream).await?;
    debug!("{}: connection opened", addr);
    let server = Arc::new(RwLock::new(ServerObj::new(&addr)));
    let canceller = server.read().await.get_canceller();
    let guard = canceller.clone().drop_guard();
    let (mut tx, mut rx) = make_channel(canceller.clone(), &addr, stream).await?;
    let (server_server, server_client) = ServerServerSharedMut::<_>::new(server.clone(), 1);

    tx.send(server_client).await?;
    let client_client = rx.recv().await?.ok_or(anyhow!("client is invalid"))?;

    tokio::spawn(server_task(
        canceller,
        addr,
        server_server,
        server_map.clone(),
    ));
    server.write().await.initialize(client_client);
    #[cfg(debug_assertions)]
    server::debug::run(server.clone()).await?;
    let mut lock_map = server_map.write().await;
    lock_map.map.insert(addr, server);
    lock_map.notify_tx.send(ConnectEvent {
        event: ConnectEventEnum::Connect,
        addr,
    })?;
    drop(lock_map);
    guard.disarm();
    info!("{}: connected", addr);
    Ok(())
}

fn tls_acceptor(config: &Config) -> Result<TlsAcceptor> {
    let certs = CertificateDer::pem_file_iter(&config.cert)?.collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_file(&config.key)?;
    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(TlsAcceptor::from(Arc::new(tls_config)))
}

pub async fn run(config: &Config, server_map: Arc<RwLock<ServerMap>>) -> Result<()> {
    let listener = TcpListener::bind(config.addr).await?;
    let acceptor = tls_acceptor(config)?;

    loop {
        if let Err(e) = accept(&listener, acceptor.clone(), server_map.clone()).await {
            warn!("accept connection error: {}", e);
        }
    }
}
