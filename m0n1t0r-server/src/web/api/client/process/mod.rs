pub mod execute;
pub mod interactive;

use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    delete, get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{client::Client, process::Agent as _};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, PartialEq)]
enum Type {
    #[serde(rename = "pid")]
    Pid,
    #[serde(rename = "name")]
    Name,
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
    let agent = client.get_process_agent().await?;
    drop(lock_obj);
    

    Ok(Json(Response::success(agent.list().await?)?))
}

#[delete("/{type}/{value}")]
pub async fn delete(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type, String)>,
) -> WebResult<impl Responder> {
    let (addr, r#type, value) = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_process_agent().await?;
    drop(lock_obj);
    

    let processes = match r#type {
        Type::Pid => agent.kill_by_pid(value.parse()?).await,
        Type::Name => agent.kill_by_name(value).await,
    }?;

    Ok(Json(Response::success(processes)?))
}
