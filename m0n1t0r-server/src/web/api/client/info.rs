use crate::{
    web::error::{Error, Result as WebResult},
    web::Response,
    ServerMap, ServerObj,
};
use actix_web::{get, web, Responder};
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
            version: lock.version().await.map_err(|_| Error::Unknown)?,
        })
    }
}

#[get("/client/{addr}/info")]
pub async fn get(
    data: web::Data<Arc<RwLock<ServerMap>>>,
    addr: web::Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock = data.read().await;
    let server = lock.get(&addr).ok_or(Error::ClientNotFound)?;

    Ok(web::Json(Response::success(
        GetInfoResponse::new(server.clone())
            .await
            .map_err(|_| Error::Unknown)?,
    )?))
}
