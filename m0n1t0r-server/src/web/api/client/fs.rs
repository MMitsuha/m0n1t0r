use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    HttpResponse, Responder, delete, get, put,
    web::{Bytes, Data, Json, Path, Query},
};
use m0n1t0r_common::{
    client::Client as _,
    fs::{Agent as _, AgentClient},
};
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Type {
    File,
    Directory,
}

#[derive(Deserialize, PartialEq)]
struct PathQuery {
    #[serde(rename = "type")]
    r#type: Type,
    path: PathBuf,
}

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Query(query): Query<PathQuery>,
) -> WebResult<impl Responder> {
    let (agent, _) = agent(data, &addr).await?;

    match query.r#type {
        Type::Directory => {
            if query.path == *"/"
                && let Ok(drives) = agent.drives().await
            {
                Ok(HttpResponse::Ok().json(Response::success(drives)?))
            } else {
                Ok(HttpResponse::Ok().json(Response::success(agent.list(query.path).await?)?))
            }
        }
        Type::File => Ok(HttpResponse::Ok().body(agent.read(query.path).await?)),
    }
}

#[delete("")]
pub async fn delete(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Query(query): Query<PathQuery>,
) -> WebResult<impl Responder> {
    let (agent, _) = agent(data, &addr).await?;

    match query.r#type {
        Type::Directory => Ok(Json(Response::success(
            agent.remove_directory(query.path).await?,
        )?)),
        Type::File => Ok(Json(Response::success(
            agent.remove_file(query.path).await?,
        )?)),
    }
}

#[put("")]
pub async fn put(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Query(query): Query<PathQuery>,
    payload: Bytes,
) -> WebResult<impl Responder> {
    let (agent, _) = agent(data, &addr).await?;

    match query.r#type {
        Type::Directory => Ok(Json(Response::success(
            agent.create_directory(query.path).await?,
        )?)),
        Type::File => Ok(Json(Response::success(
            agent.write(query.path, payload.to_vec()).await?,
        )?)),
    }
}

pub mod metadata {
    use super::*;

    #[get("/metadata")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        Query(query): Query<PathQuery>,
    ) -> WebResult<impl Responder> {
        let (agent, _) = agent(data, &addr).await?;

        match query.r#type {
            Type::Directory => Err(Error::Unimplemented),
            Type::File => Ok(Json(Response::success(agent.file(query.path).await?)?)),
        }
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
