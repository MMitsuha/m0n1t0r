pub mod friend;
pub mod url;

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
    client::Client as _,
    qq::{Agent as _, AgentClient},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let (agent, _) = get_agent(data, &addr).await?;
    Ok(Json(Response::success(agent.list().await?)?))
}

pub async fn get_agent(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
) -> WebResult<(AgentClient, CancellationToken)> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let canceller = lock_obj.get_canceller();
    let agent = client.get_qq_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
