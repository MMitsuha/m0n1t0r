pub mod execute;
pub mod interactive;

use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{client::Client, process::Agent as _};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let lock_map = data.read().await;
    let server = lock_map.get(&addr).ok_or(Error::ClientNotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_process_agent().await?;

    Ok(Json(Response::success(agent.list().await?)?))
}
