use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap, ServerObj,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::client::{Client as _, TargetPlatform};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

#[derive(Serialize)]
pub struct Get {
    addr: SocketAddr,
    version: String,
    target_platform: TargetPlatform,
}

impl Get {
    pub async fn new(server: Arc<RwLock<ServerObj>>) -> WebResult<Self> {
        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Self {
            addr: lock_obj.get_addr().clone(),
            version: client.version().await?,
            target_platform: client.target_platform().await?,
        })
    }
}

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = data.read().await;
    let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

    Ok(Json(Response::success(Get::new(server.clone()).await?)?))
}

pub mod update {
    use super::*;

    #[get("/update/{url}")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        path: Path<(SocketAddr, Url)>,
    ) -> WebResult<impl Responder> {
        let (addr, url) = path.into_inner();
        let lock_map = data.read().await;
        let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.update(url).await?)?))
    }
}
