use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{
    client::{Client as _, TargetPlatform},
    info,
};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

#[derive(Serialize)]
pub struct Detail {
    addr: SocketAddr,
    version: String,
    target_platform: TargetPlatform,
    system_info: info::System,
}

impl Detail {
    pub async fn new(
        addr: &SocketAddr,
        version: String,
        target_platform: TargetPlatform,
        system_info: info::System,
    ) -> WebResult<Self> {
        Ok(Self {
            addr: addr.clone(),
            version,
            target_platform,
            system_info,
        })
    }
}

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;

    Ok(Json(Response::success(
        Detail::new(
            lock_obj.get_addr(),
            client.version().await?,
            client.target_platform().await?,
            client.system_info().await?,
        )
        .await?,
    )?))
}

pub mod update {
    use super::*;

    #[get("/update/{url}")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        path: Path<(SocketAddr, Url)>,
    ) -> WebResult<impl Responder> {
        let (addr, url) = path.into_inner();
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.update(url).await?)?))
    }
}

pub mod notify {
    use crate::web::util;
    use actix_web::{web::Payload, HttpRequest};
    use actix_ws::Message;
    use tokio::{select, task};

    use super::*;

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
}
