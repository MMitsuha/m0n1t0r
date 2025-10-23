use crate::{
    ServerMap,
    web::{Response, Result as WebResult, api::client::qq},
};
use actix_web::{
    Responder, get,
    web::{Data, Json, Path},
};
use m0n1t0r_common::qq::Agent as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[get("/{id}/friend")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, i64)>,
) -> WebResult<impl Responder> {
    let (addr, id) = path.into_inner();
    let (agent, _) = qq::agent(data, &addr).await?;

    Ok(Json(Response::success(agent.friends(id).await?)?))
}
