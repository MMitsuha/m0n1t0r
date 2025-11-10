pub mod execute;
pub mod interactive;

use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, delete, get,
    web::{Data, Json, Path, Query},
};
use m0n1t0r_common::{
    client::Client as _,
    process::{Agent as _, AgentClient},
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Type {
    Pid,
    Name,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Execute {
    Blocked,
    Detached,
}

impl Default for Execute {
    fn default() -> Self {
        Self::Blocked
    }
}

#[derive(Deserialize)]
struct CommandForm {
    command: String,
    #[serde(default)]
    option: Execute,
}

#[derive(Deserialize)]
struct ProcessQuery {
    #[serde(rename = "type")]
    r#type: Type,
}

#[get("")]
pub async fn all(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let (agent, _) = agent(data, &addr).await?;

    Ok(Json(Response::success(agent.list().await?)?))
}

#[delete("/{value}")]
pub async fn delete(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, String)>,
    Query(query): Query<ProcessQuery>,
) -> WebResult<impl Responder> {
    let (addr, value) = path.into_inner();
    let (agent, _) = agent(data, &addr).await?;

    let processes = match query.r#type {
        Type::Pid => agent.kill_by_id(value.parse()?).await,
        Type::Name => agent.kill_by_name(value).await,
    }?;

    Ok(Json(Response::success(processes)?))
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
    let agent = client.process_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
