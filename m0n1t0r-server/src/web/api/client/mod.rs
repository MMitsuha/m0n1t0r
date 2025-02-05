pub mod client;
pub mod fs;
pub mod info;
pub mod network;
pub mod process;
pub mod proxy;
pub mod qq;
pub mod update;

use crate::{
    web::{Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Payload},
    HttpRequest, Responder,
};
use actix_ws::Message;
use std::sync::Arc;
use tokio::{select, sync::RwLock, task};

#[get("")]
pub async fn get(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let mut details = Vec::new();

    for (addr, server) in lock_map.iter() {
        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        details.push(client::Detail::new(addr, client).await?);
    }
    Ok(Json(Response::success(details)?))
}

pub mod notify {
    use super::*;
    use crate::web::util;

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
}
