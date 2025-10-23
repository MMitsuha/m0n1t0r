use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_web::{
    Responder, get,
    web::{Data, Json, Path},
};
use m0n1t0r_common::client::Client as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub mod system {
    use super::*;

    #[get("/system")]
    pub async fn get(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
    ) -> WebResult<impl Responder> {
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.client()?;

        Ok(Json(Response::success(client.system_info().await?)?))
    }
}
