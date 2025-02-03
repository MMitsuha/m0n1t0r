use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use m0n1t0r_common::{client::Client as _, process::Agent as _};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[get("/execute/{command}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, String)>,
) -> WebResult<impl Responder> {
    let (addr, command) = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_process_agent().await?;
    drop(lock_obj);

    let mut command = shell_words::split(&command)?;
    let program = command.remove(0);

    Ok(Json(Response::success(
        agent.execute(program, command).await?,
    )?))
}
