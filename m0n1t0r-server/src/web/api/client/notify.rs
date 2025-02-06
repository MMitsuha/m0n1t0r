use crate::{
    web::{util, Error, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::Message;
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[get("/notify")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let canceller = lock_obj.get_canceller();
    drop(lock_obj);

    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                _ = canceller.cancelled() => break,
            }
        }
        Ok::<_, anyhow::Error>(())
    }));
    Ok(response)
}
