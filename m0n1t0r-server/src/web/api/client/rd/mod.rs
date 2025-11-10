pub mod stream;

use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, get,
    web::{Data, Json, Path},
};
use m0n1t0r_common::{
    client::Client as _,
    rd::{Agent as _, AgentClient},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[get("")]
pub async fn all(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let (agent, _) = agent(data, &addr).await?;

    Ok(Json(Response::success(agent.displays().await?)?))
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
    let agent = client.rd_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
