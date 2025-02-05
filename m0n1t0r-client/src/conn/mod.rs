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
    rch::base::{Receiver as RemoteReceiver, Sender as RemoteSender},
    Cfg, Connect,
};
use rustls::RootCertStore;
#[allow(unused_imports)]
use rustls_pki_types::{pem::PemObject as _, CertificateDer, DnsName, ServerName};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{io, net::TcpStream, select, sync::RwLock, time};
use tokio_rustls::{client::TlsStream, TlsConnector};
use tokio_util::sync::CancellationToken;

pub struct Config {
    host: String,
    addr: SocketAddr,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
        Self {
            host: config.host.clone(),
            addr: config.addr.clone(),
        }
    }
}

/// Connect to a client and create a channel for client exchange.
async fn make_channel<'transport>(
    canceller: CancellationToken,
    addr: &SocketAddr,
    stream: TlsStream<TcpStream>,
) -> Result<(RemoteSender<ClientClient>, RemoteReceiver<ServerClient>)> {
    let addr = addr.clone();
    let (socket_rx, socket_tx) = io::split(stream);
    let (conn, tx, rx): (_, RemoteSender<ClientClient>, RemoteReceiver<ServerClient>) =
        Connect::io(
            Cfg {
                shared_send_queue: 64,
                transport_receive_queue: 64,
                transport_send_queue: 64,
                receive_buffer: 0x3200000,
                chunk_size: 0x8000,
                max_data_size: 0x3200000,
                ..Default::default()
            },
            socket_rx,
            socket_tx,
        )
        .await?;

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

fn ca_store() -> Result<RootCertStore> {
    let mut store = RootCertStore::empty();

    for cert in CertificateDer::pem_slice_iter(include_bytes!(concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "certs/ca.crt"
    ))) {
        store.add(cert?)?;
    }

    Ok(store)
}

#[cfg(debug_assertions)]
mod debug {
    use rustls::client::danger::ServerCertVerifier;

    #[derive(Default, Debug)]
    pub struct NoCertificateVerification {}

    impl ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls_pki_types::CertificateDer<'_>,
            _intermediates: &[rustls_pki_types::CertificateDer<'_>],
            _server_name: &rustls_pki_types::ServerName<'_>,
            _ocsp_response: &[u8],
            _now: rustls_pki_types::UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &rustls_pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &rustls_pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            rustls::crypto::ring::default_provider()
                .signature_verification_algorithms
                .supported_schemes()
        }
    }
}

fn tls_connector() -> Result<TlsConnector> {
    let store = ca_store()?;
    #[allow(unused_mut)]
    let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(store)
        .with_no_client_auth();
    #[cfg(debug_assertions)]
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(debug::NoCertificateVerification::default()));

    Ok(TlsConnector::from(Arc::new(config)))
}

pub async fn accept(
    addr: &SocketAddr,
    #[allow(unused_variables)] host: &str,
    connector: TlsConnector,
    client_map: Arc<RwLock<ClientMap>>,
) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let stream = connector
        .connect(
            #[cfg(not(debug_assertions))]
            ServerName::DnsName(DnsName::try_from(host.to_string())?),
            #[cfg(debug_assertions)]
            ServerName::IpAddress(addr.ip().into()),
            stream,
        )
        .await?;
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
        addr.clone(),
        client_server,
        client_map.clone(),
    ));
    client.write().await.initialize(server_client);
    client_map.write().await.insert(addr.clone(), client);
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

pub async fn run(config: &Config, client_map: Arc<RwLock<ClientMap>>) -> Result<()> {
    let connector = tls_connector()?;

    while let Err(e) = accept(
        &config.addr,
        &config.host,
        connector.clone(),
        client_map.clone(),
    )
    .await
    {
        warn!("failed to connect server: {}", e);
        time::sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}
