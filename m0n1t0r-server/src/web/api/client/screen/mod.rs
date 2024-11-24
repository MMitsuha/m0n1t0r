use crate::{
    web::{Error, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::Message;
use anyhow::anyhow;
use m0n1t0r_common::{
    client::Client,
    screen::{Agent, Options},
};
use scap::capturer::Resolution;
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    let canceller = lock_obj.get_canceller();
    drop(lock_obj);
    

    if agent.availability().await? == false {
        return Err(Error::UnsupportedError);
    }

    let mut rx = agent
        .record(Options {
            fps: 20,
            show_cursor: true,
            show_highlight: true,
            output_resolution: Resolution::_720p,
            ..Default::default()
        })
        .await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(async move {
        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                frame = rx.recv() => session.binary(frame?.ok_or(anyhow!("no frame received"))?).await?,
                _ = canceller.cancelled() => break,
            }
        }
        session.close(None).await?;
        Ok::<_, anyhow::Error>(())
    });
    Ok(response)
}
