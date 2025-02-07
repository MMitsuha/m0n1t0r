use crate::{
    web::{
        api::client::process::{self, CommandForm},
        util, Result as WebResult,
    },
    ServerMap,
};
use actix_web::{
    get,
    web::{Buf, Data, Path, Payload, Query},
    HttpRequest, Responder,
};
use actix_ws::Message;
use anyhow::anyhow;
use m0n1t0r_common::process::Agent as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[get("/interactive")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    query: Query<CommandForm>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let query = query.into_inner();
    let (agent, canceller) = process::get_agent(data, &addr).await?;

    let (stdin_tx, stdout_rx, stderr_rx) = agent.interactive(query.command).await?;
    let mut stdin_tx = stdin_tx.into_inner().await?;
    let mut stdout_rx = stdout_rx.into_inner().await?;
    let mut stderr_rx = stderr_rx.into_inner().await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Text(msg) => stdin_tx.send(msg.into_bytes()).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                msg = stdout_rx.recv() => session.text(String::from_utf8_lossy(msg?.ok_or(anyhow!("channel closed"))?.chunk()).to_string()).await?,
                msg = stderr_rx.recv() => session.text(String::from_utf8_lossy(msg?.ok_or(anyhow!("channel closed"))?.chunk()).to_string()).await?,
                _ = stdin_tx.closed() => break,
                _ = canceller.cancelled() => break,
            }
        }
        Ok::<_, anyhow::Error>(())
    }));

    Ok(response)
}
