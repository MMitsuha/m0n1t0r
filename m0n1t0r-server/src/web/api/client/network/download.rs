use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{client::Client as _, network::Agent as _};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

#[get("/download/{url}/{path}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Url, PathBuf)>,
) -> WebResult<impl Responder> {
    let (addr, url, path) = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_network_agent().await?;
    drop(lock_obj);

    Ok(Json(Response::success(agent.download(url, path).await?)?))
}
