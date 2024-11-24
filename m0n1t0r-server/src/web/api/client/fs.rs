use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    delete, get, head, put,
    web::{Bytes, Data, Json, Path},
    HttpResponse, Responder,
};
use m0n1t0r_common::{client::Client, fs::Agent as _};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

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
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_fs_agent().await?;
    drop(lock_obj);

    if r#type == Type::Directory {
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
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_fs_agent().await?;
    drop(lock_obj);

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
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_fs_agent().await?;
    drop(lock_obj);

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

#[head("/{type}/{path}")]
pub async fn head(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type, PathBuf)>,
) -> WebResult<impl Responder> {
    let (addr, _, path) = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_fs_agent().await?;
    drop(lock_obj);

    Ok(Json(Response::success(agent.file(path).await?)?))
}
