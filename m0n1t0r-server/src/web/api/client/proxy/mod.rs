pub mod socks5;

use crate::{
    web::{error::Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    delete, get,
    web::{Data, Json, Path},
    Responder,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Type {
    Socks5,
}

#[derive(Serialize, Deserialize)]
struct Detail {
    addr: SocketAddr,
    r#type: Type,
}

lazy_static! {
    static ref PROXY_MAP: Arc<RwLock<HashMap<SocketAddr, (CancellationToken, Type)>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

pub async fn close(addr: &SocketAddr) -> WebResult<()> {
    PROXY_MAP
        .read()
        .await
        .get(addr)
        .ok_or(Error::NotFound)?
        .0
        .cancel();
    Ok(())
}

#[get("")]
pub async fn get(_: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(Json(Response::success(Detail::new().await)?))
}

#[delete("/{addr}")]
pub async fn delete(
    _: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    Ok(Json(Response::success(close(&addr).await?)?))
}

impl Detail {
    pub async fn new() -> Vec<Self> {
        PROXY_MAP
            .read()
            .await
            .iter()
            .map(|(addr, (_, r#type))| Self {
                addr: *addr,
                r#type: *r#type,
            })
            .collect()
    }
}
