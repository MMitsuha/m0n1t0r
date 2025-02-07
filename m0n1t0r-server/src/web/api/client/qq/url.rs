use crate::{
    web::{api::client::qq, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::qq::Agent as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[get("/{id}/url")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, i64)>,
) -> WebResult<impl Responder> {
    let (addr, id) = path.into_inner();
    let (agent, _) = qq::get_agent(data, &addr).await?;

    Ok(Json(Response::success(agent.urls(id).await?)?))
}
