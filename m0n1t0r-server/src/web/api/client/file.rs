use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use m0n1t0r_common::{client::Client, file::Agent as _};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[get("/client/{addr}/file/{path}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    param: Path<(SocketAddr, PathBuf)>,
) -> WebResult<impl Responder> {
    let (addr, path) = param.into_inner();
    let lock = data.read().await;
    let server = lock.get(&addr).ok_or(Error::ClientNotFound)?;

    let lock = server.read().await;
    let client = lock.get_client()?;
    let agent = client.get_file_agent().await?;
    let metadata = agent.file(path.clone()).await?;
    if metadata.is_dir {
        Ok(HttpResponse::Ok().json(Response::success(agent.list(path).await?)?))
    } else {
        Ok(HttpResponse::Ok().body(agent.read(path).await?))
    }
}
