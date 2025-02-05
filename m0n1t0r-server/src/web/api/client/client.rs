use crate::{
    web::{util, Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get, post,
    web::{Data, Form, Json, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::Message;
use m0n1t0r_common::{
    client::{Client as _, ClientClient, TargetPlatform},
    info,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{select, sync::RwLock, task};
use url::Url;

#[derive(Serialize)]
pub struct Detail {
    addr: SocketAddr,
    version: String,
    target_platform: TargetPlatform,
    system_info: info::System,
    build_time: String,
    commit_hash: String,
}

impl Detail {
    pub async fn new(addr: &SocketAddr, client: &ClientClient) -> WebResult<Self> {
        Ok(Self {
            addr: addr.clone(),
            version: client.version().await?,
            target_platform: client.target_platform().await?,
            system_info: client.system_info().await?,
            build_time: client.build_time().await?,
            commit_hash: client.commit_hash().await?,
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
        Detail::new(lock_obj.get_addr(), client).await?,
    )?))
}

pub mod environment {
    use super::*;

    #[get("/environment")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
    ) -> WebResult<impl Responder> {
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.environment().await?)?))
    }
}

pub mod terminate {
    use super::*;

    #[post("/terminate")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
    ) -> WebResult<impl Responder> {
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.terminate().await?)?))
    }
}

pub mod notify {
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
