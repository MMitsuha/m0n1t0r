use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, get,
    web::{Data, Json, Path},
};
use m0n1t0r_common::{
    client::{Client as _, ClientClient, TargetPlatform},
    info,
};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Serialize)]
pub struct Detail {
    addr: SocketAddr,
    version: String,
    target_platform: TargetPlatform,
    system_info: info::System,
    build_time: String,
    commit_hash: String,
    current_exe: PathBuf,
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
            current_exe: client.current_exe().await?,
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
    let client = lock_obj.client()?;

    Ok(Json(Response::success(
        Detail::new(lock_obj.addr(), client).await?,
    )?))
}
