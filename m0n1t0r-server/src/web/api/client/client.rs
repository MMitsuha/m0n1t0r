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
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

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
        let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.update(url).await?)?))
    }
}
