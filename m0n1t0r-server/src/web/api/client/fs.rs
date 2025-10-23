use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    HttpResponse, Responder, delete, get, put,
    web::{Bytes, Data, Json, Path},
};
use m0n1t0r_common::{
    client::Client as _,
    fs::{Agent as _, AgentClient},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Serialize, Deserialize, PartialEq)]
enum Type {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "dir")]
    Directory,
}

#[get("/{type}/{path}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type, PathBuf)>,
) -> WebResult<impl Responder> {
    let (addr, r#type, path) = path.into_inner();
    let (agent, _) = agent(data, &addr).await?;

    if r#type == Type::Directory {
        if path == PathBuf::from("/")
            && let Ok(drives) = agent.drives().await {
                return Ok(HttpResponse::Ok().json(Response::success(drives)?));
            }

        Ok(HttpResponse::Ok().json(Response::success(agent.list(path).await?)?))
    } else {
        Ok(HttpResponse::Ok().body(agent.read(path).await?))
    }
}

#[delete("/{type}/{path}")]
pub async fn delete(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type, PathBuf)>,
) -> WebResult<impl Responder> {
    let (addr, r#type, path) = path.into_inner();
    let (agent, _) = agent(data, &addr).await?;

    if r#type == Type::Directory {
        Ok(Json(Response::success(
            agent.remove_directory(path).await?,
        )?))
    } else {
        Ok(Json(Response::success(agent.remove_file(path).await?)?))
    }
}

#[put("/{type}/{path}")]
pub async fn put(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type, PathBuf)>,
    payload: Bytes,
) -> WebResult<impl Responder> {
    let (addr, r#type, path) = path.into_inner();
    let (agent, _) = agent(data, &addr).await?;

    if r#type == Type::Directory {
        Ok(Json(Response::success(
            agent.create_directory(path).await?,
        )?))
    } else {
        Ok(Json(Response::success(
            agent.write(path, payload.to_vec()).await?,
        )?))
    }
}

pub mod metadata {
    use super::*;

    #[get("/metadata/{path}")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        path: Path<(SocketAddr, PathBuf)>,
    ) -> WebResult<impl Responder> {
        let (addr, path) = path.into_inner();
        let (agent, _) = agent(data, &addr).await?;

        Ok(Json(Response::success(agent.file(path).await?)?))
    }
}

pub async fn agent(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
) -> WebResult<(AgentClient, CancellationToken)> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.client()?;
    let canceller = lock_obj.canceller();
    let agent = client.fs_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
