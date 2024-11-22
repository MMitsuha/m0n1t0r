use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
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
        let lock_map = data.read().await;
        let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(client.system_info().await?)?))
    }
}
