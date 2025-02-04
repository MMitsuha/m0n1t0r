pub mod download;

use crate::{
    web::{Error, Result as WebResult},
    ServerMap,
};
use actix_web::web::Data;
use m0n1t0r_common::{client::Client as _, network::AgentClient};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub async fn get_agent(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: &SocketAddr,
) -> WebResult<(AgentClient, CancellationToken)> {
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let canceller = lock_obj.get_canceller();
    let agent = client.get_network_agent().await?;
    drop(lock_obj);

    Ok((agent, canceller))
}
