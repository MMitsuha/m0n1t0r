use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap, ServerObj,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::server::Server;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[derive(Serialize)]
pub struct GetInfoResponse {
    addr: SocketAddr,
    version: String,
}

impl GetInfoResponse {
    pub async fn new(server: Arc<RwLock<ServerObj>>) -> WebResult<Self> {
        let lock = server.read().await;

        Ok(Self {
            addr: lock.get_addr().clone(),
            version: lock.version().await?,
        })
    }
}

#[get("/client/{addr}/info")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock = data.read().await;
    let server = lock.get(&addr).ok_or(Error::ClientNotFound)?;

    Ok(Json(Response::success(
        GetInfoResponse::new(server.clone()).await?,
    )?))
}
