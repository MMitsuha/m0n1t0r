use crate::{
    ServerMap,
    web::{Result as WebResult, util},
};
use actix_web::{
    HttpRequest, Responder, get,
    web::{Data, Payload},
};
use actix_ws::Message;
use std::sync::Arc;
use tokio::{select, sync::RwLock, task};

#[get("/notify")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let lock_map = &data.read().await;
    let mut rx = lock_map.notify_rx.clone();
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        rx.mark_unchanged();
        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                _ = rx.changed() => session.text(serde_json::to_string(&*rx.borrow_and_update())?).await?,
            }
        }
        Ok::<_, anyhow::Error>(())
    }));
    Ok(response)
}
