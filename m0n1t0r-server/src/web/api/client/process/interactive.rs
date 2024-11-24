use crate::{
    web::{Error, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Buf, Data, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::Message;
use anyhow::anyhow;
use m0n1t0r_common::{client::Client, process::Agent as _};
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[get("/interactive/{command}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, String)>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let (addr, command) = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_process_agent().await?;
    let canceller = lock_obj.get_canceller();
    drop(lock_obj);

    let (stdin_tx, stdout_rx, stderr_rx) = agent.interactive(command).await?;
    let mut stdin_tx = stdin_tx.into_inner().await?;
    let mut stdout_rx = stdout_rx.into_inner().await?;
    let mut stderr_rx = stderr_rx.into_inner().await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(async move {
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
        session.close(None).await?;
        Ok::<_, anyhow::Error>(())
    });
    Ok(response)
}
