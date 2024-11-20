use crate::{
    web::{error::Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path, Query},
    Responder,
};
use anyhow::{anyhow, Result};
use as_any::Downcast;
use m0n1t0r_common::{
    client::Client,
    proxy::{Agent, AgentClient},
};
use remoc::chmux::ReceiverStream;
use serde::{Deserialize, Serialize};
use socks5_impl::{
    protocol::{Address, Reply},
    server::{
        auth::{NoAuth, UserKeyAuth},
        ClientConnection, IncomingConnection, Server,
    },
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io,
    net::{self},
    select,
    sync::RwLock,
};
use tokio_util::{
    io::{CopyToBytes, SinkWriter, StreamReader},
    sync::CancellationToken,
};

#[derive(Serialize, Deserialize, PartialEq)]
struct User {
    name: String,
    password: String,
}

pub mod pass {
    pub use super::*;

    #[get("/socks5/pass")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        user: Query<User>,
    ) -> WebResult<impl Responder> {
        let lock_map = data.read().await;
        let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;
        let agent = Arc::new(client.get_proxy_agent().await?);
        let canceller = lock_obj.get_canceller();
        drop(lock_obj);
        drop(lock_map);

        let auth = Arc::new(UserKeyAuth::new(&user.name, &user.password));
        let listener = Server::bind("0.0.0.0:0".parse()?, auth).await?;
        let addr = listener.local_addr()?;
        tokio::spawn(async move {
            loop {
                let agent = agent.clone();
                select! {
                    accept = listener.accept() => match accept {
                        Ok((conn, _)) => { tokio::spawn(handle(conn, agent, canceller.clone())); },
                        Err(_) => continue,
                    },
                    _ = canceller.cancelled() => break,
                }
            }

            Ok::<_, anyhow::Error>(())
        });

        Ok(Json(Response::success(addr)?))
    }
}

pub mod noauth {
    pub use super::*;

    #[get("/socks5/noauth")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
    ) -> WebResult<impl Responder> {
        let lock_map = data.read().await;
        let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;
        let agent = Arc::new(client.get_proxy_agent().await?);
        let canceller = lock_obj.get_canceller();
        drop(lock_obj);
        drop(lock_map);

        let auth = Arc::new(NoAuth::default());
        let listener = Server::bind("0.0.0.0:0".parse()?, auth).await?;
        let addr = listener.local_addr()?;
        tokio::spawn(async move {
            loop {
                let agent = agent.clone();
                select! {
                    accept = listener.accept() => match accept {
                        Ok((conn, _)) => { tokio::spawn(handle(conn, agent, canceller.clone())); },
                        Err(_) => continue,
                    },
                    _ = canceller.cancelled() => break,
                }
            }

            Ok::<_, anyhow::Error>(())
        });

        Ok(Json(Response::success(addr)?))
    }
}

async fn handle<S>(
    conn: IncomingConnection<S>,
    agent: Arc<AgentClient>,
    canceller: CancellationToken,
) -> Result<()>
where
    S: Send + Sync + 'static,
{
    let (conn, auth) = conn.authenticate().await?;

    if let Some(auth) = auth.downcast_ref::<std::io::Result<bool>>() {
        match auth {
            Ok(b) => {
                if *b == false {
                    return Err(anyhow!("auth failed"));
                }
            }
            Err(e) => return Err(serde_error::Error::new(e).into()),
        }
    }

    match conn.wait_request().await? {
        ClientConnection::UdpAssociate(associate, _) => {
            let mut conn = associate
                .reply(Reply::CommandNotSupported, Address::unspecified())
                .await?;
            conn.shutdown().await?;
        }
        ClientConnection::Bind(bind, _) => {
            let mut conn = bind
                .reply(Reply::CommandNotSupported, Address::unspecified())
                .await?;
            conn.shutdown().await?;
        }
        ClientConnection::Connect(connect, addr) => {
            let addr: SocketAddr = match addr {
                Address::DomainAddress(domain, port) => net::lookup_host((domain, port))
                    .await?
                    .next()
                    .ok_or(anyhow!("dns lookup failed"))?,
                Address::SocketAddress(addr) => addr,
            };
            let (tx, rx) = agent.connect(addr).await?;
            let mut rx = StreamReader::new(ReceiverStream::new(rx.into_inner().await?));
            let mut tx = SinkWriter::new(CopyToBytes::new(tx.into_inner().await?.into_sink()));
            let mut conn = connect
                .reply(Reply::Succeeded, Address::unspecified())
                .await?;
            let (mut conn_rx, mut conn_tx) = conn.split();

            select! {
                _ = io::copy(&mut rx, &mut conn_tx) => {},
                _ = io::copy(&mut conn_rx, &mut tx) => {},
                _ = canceller.cancelled() => {},
            };
        }
    }
    Ok(())
}
