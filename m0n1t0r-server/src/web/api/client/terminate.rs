use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, post,
    web::{Data, Json, Path},
};
use m0n1t0r_common::client::Client as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[post("/terminate")]
pub async fn post(
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
