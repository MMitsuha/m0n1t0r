use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    Responder,
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
    let client = lock_obj.get_client()?;

    let _ = client.terminate().await;

    Ok(Json(Response::success(())?))
}
