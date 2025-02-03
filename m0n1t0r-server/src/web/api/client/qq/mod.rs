use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{client::Client as _, qq::Agent as _};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_qq_agent().await?;
    drop(lock_obj);

    Ok(Json(Response::success(agent.list().await?)?))
}

pub mod urls {
    pub use super::*;

    #[get("/{id}/urls")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        path: Path<(SocketAddr, i64)>,
    ) -> WebResult<impl Responder> {
        let (addr, id) = path.into_inner();
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;
        let agent = client.get_qq_agent().await?;
        drop(lock_obj);

        Ok(Json(Response::success(agent.urls(id).await?)?))
    }
}
