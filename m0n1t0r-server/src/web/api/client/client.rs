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
pub struct Get {
    addr: SocketAddr,
    version: String,
}

impl Get {
    pub async fn new(server: Arc<RwLock<ServerObj>>) -> WebResult<Self> {
        let lock_obj = server.read().await;

        Ok(Self {
            addr: lock_obj.get_addr().clone(),
            version: lock_obj.version().await?,
        })
    }
}

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = data.read().await;
    let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

    Ok(Json(Response::success(Get::new(server.clone()).await?)?))
}
