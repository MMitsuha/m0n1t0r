use crate::{
    ServerMap,
    web::{
        Response, Result as WebResult,
        api::{client::proxy, global::proxy::*},
        error::Error,
    },
};
use actix_web::{
    Responder, post,
    web::{Data, Form, Json, Path},
};
use anyhow::anyhow;
use m0n1t0r_common::proxy::Agent as _;
use remoc::chmux::ReceiverStream;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::{io, net::TcpStream, select, sync::RwLock};
use tokio_util::{
    io::{CopyToBytes, SinkWriter, StreamReader},
    sync::CancellationToken,
};

#[derive(Deserialize)]
struct ForwardForm {
    from: SocketAddr,
    to: SocketAddr,
}

#[post("/forward")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Form(form): Form<ForwardForm>,
) -> WebResult<impl Responder> {
    Ok(Json(Response::success(
        open(data, &addr, form.from, form.to).await?,
    )?))
}

pub async fn open_internal(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
    from: SocketAddr,
    to: SocketAddr,
) -> WebResult<(CancellationToken, CancellationToken)> {
    let (agent, canceller_global1) = proxy::agent(data, addr).await?;
    let canceller_global2 = canceller_global1.clone();
    let agent = Arc::new(agent);

    let (mut my_rx, mut canceller_tx) = agent.forward(to).await?;
    let canceller_scoped1 = CancellationToken::new();
    let canceller_scoped2 = canceller_scoped1.clone();

    tokio::spawn(async move {
        loop {
            select! {
                received = my_rx.recv() => {
                    let (tx, rx, _) = received?.ok_or(anyhow!("forward is invalid"))?;
                    let (mut stream_rx, mut stream_tx) = TcpStream::connect(from).await?.into_split();

                    tokio::spawn(async move {
                        let mut rx = StreamReader::new(ReceiverStream::new(rx.into_inner().await?));
                        let mut tx = SinkWriter::new(CopyToBytes::new(tx.into_inner().await?.into_sink()));

                        select! {
                            _ = io::copy(&mut rx, &mut stream_tx) => {},
                            _ = io::copy(&mut stream_rx, &mut tx) => {},
                        }
                        Ok::<_, Error>(())
                    });
                },
                _ = canceller_global2.cancelled() => break,
                _ = canceller_scoped2.cancelled() => break,
            }
        }
        canceller_tx.send(()).await?;
        Ok::<_, anyhow::Error>(())
    });
    Ok((canceller_global1, canceller_scoped1))
}

pub async fn open(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
    from: SocketAddr,
    to: SocketAddr,
) -> WebResult<()> {
    let (canceller_global1, canceller_scoped1) = open_internal(data, addr, from, to).await?;
    let canceller_scoped2 = canceller_scoped1.clone();
    let key = PROXY_MAP.write().await.insert(Proxy::new(
        Type::Forward((from, to, addr).into()),
        canceller_scoped2,
    ));

    tokio::spawn(async move {
        select! {
            _ = canceller_global1.cancelled() => {},
            _ = canceller_scoped1.cancelled() => {},
        }
        PROXY_MAP.write().await.remove(key);
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
