use super::{Type, PROXY_MAP};
use crate::{
    web::{api::client::proxy, error::Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    Responder,
};
use anyhow::anyhow;
use as_any::Downcast;
use m0n1t0r_common::proxy::{Agent, AgentClient};
use remoc::chmux::ReceiverStream;
use serde::{Deserialize, Serialize};
use socks5_impl::{
    protocol::{Address, Reply},
    server::{
        auth::{NoAuth, UserKeyAuth},
        AuthAdaptor, ClientConnection, IncomingConnection, Server,
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
struct PassForm {
    name: String,
    password: String,
}

pub async fn open_internal<O>(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
    listen: SocketAddr,
    auth: AuthAdaptor<O>,
) -> WebResult<(SocketAddr, CancellationToken, CancellationToken)>
where
    O: 'static + Sync + Sync + Send,
{
    let (agent, canceller_global1) = proxy::get_agent(data, addr).await?;
    let canceller_global2 = canceller_global1.clone();
    let agent = Arc::new(agent);

    let listener = Server::bind(listen, auth).await?;
    let addr = listener.local_addr()?;
    let canceller_scoped1 = CancellationToken::new();
    let canceller_scoped2 = canceller_scoped1.clone();

    tokio::spawn(async move {
        loop {
            let agent = agent.clone();
            select! {
                accept = listener.accept() => match accept {
                    Ok((conn, _)) => { tokio::spawn(handle(conn, agent, canceller_global2.clone(), canceller_scoped2.clone())); },
                    Err(_) => continue,
                },
                _ = canceller_global2.cancelled() => break,
                _ = canceller_scoped2.cancelled() => break,
            }
        }
        Ok::<_, anyhow::Error>(())
    });
    Ok((addr, canceller_global1, canceller_scoped1))
}

pub async fn open<O>(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
    listen: SocketAddr,
    auth: AuthAdaptor<O>,
) -> WebResult<SocketAddr>
where
    O: 'static + Sync + Sync + Send,
{
    let (addr, canceller_global1, canceller_scoped1) =
        open_internal(data, addr, listen, auth).await?;
    let canceller_scoped2 = canceller_scoped1.clone();

    tokio::spawn(async move {
        loop {
            select! {
                _ = canceller_global1.cancelled() => break,
                _ = canceller_scoped1.cancelled() => break,
            }
        }
        PROXY_MAP.write().await.remove(&addr);
        Ok::<_, anyhow::Error>(())
    });

    PROXY_MAP
        .write()
        .await
        .insert(addr, (canceller_scoped2, Type::Socks5));

    Ok(addr)
}

pub mod pass {
    use actix_web::web::Form;

    pub use super::*;

    #[post("/socks5/pass")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        form: Form<PassForm>,
    ) -> WebResult<impl Responder> {
        let auth = Arc::new(UserKeyAuth::new(&form.name, &form.password));
        let addr = open(data, &addr, "0.0.0.0:0".parse().unwrap(), auth).await?;

        Ok(Json(Response::success(addr)?))
    }
}

pub mod noauth {
    pub use super::*;

    #[post("/socks5/noauth")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
    ) -> WebResult<impl Responder> {
        let auth = Arc::new(NoAuth::default());
        let addr = open(data, &addr, "0.0.0.0:0".parse().unwrap(), auth).await?;

        Ok(Json(Response::success(addr)?))
    }
}

async fn handle<S>(
    conn: IncomingConnection<S>,
    agent: Arc<AgentClient>,
    canceller_global: CancellationToken,
    canceller_scoped: CancellationToken,
) -> WebResult<()>
where
    S: Send + Sync + 'static,
{
    let (conn, auth) = conn.authenticate().await?;

    if let Some(auth) = auth.downcast_ref::<std::io::Result<bool>>() {
        match auth {
            Ok(b) => {
                if *b == false {
                    return Err(Error::Socks5AuthFailed(serde_error::Error::new(&*anyhow!(
                        "password or username mismatch"
                    )))
                    .into());
                }
            }
            Err(e) => return Err(Error::Socks5AuthFailed(serde_error::Error::new(e))),
        }
    }

    match conn.wait_request().await? {
        // TODO: Implement this
        ClientConnection::UdpAssociate(associate, _) => {
            let mut conn = associate
                .reply(Reply::CommandNotSupported, Address::unspecified())
                .await?;
            conn.shutdown().await?;
        }
        // TODO: Implement this
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
                _ = canceller_global.cancelled() => {},
                _ = canceller_scoped.cancelled() => {},
            };
        }
    }
    Ok(())
}
