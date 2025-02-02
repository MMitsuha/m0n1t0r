pub mod socks5;

use crate::{
    web::{error::Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    delete, get,
    web::{Data, Json, Path, Query},
    Responder,
};
use anyhow::{anyhow, bail, Result};
use as_any::Downcast;
use lazy_static::lazy_static;
use m0n1t0r_common::{
    client::Client,
    proxy::{Agent, AgentClient},
};
use remoc::chmux::ReceiverStream;
use serde::{Deserialize, Serialize};
use socks5_impl::{
    protocol::{Address, Reply},
    server::{
        auth::{NoAuth, UserKeyAuth},
        AuthAdaptor, ClientConnection, IncomingConnection, Server,
    },
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    io,
    net::{self},
    select,
    sync::RwLock,
};
use tokio_util::{
    io::{CopyToBytes, SinkWriter, StreamReader},
    sync::CancellationToken,
};

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
    static ref CANCEL_MAP: Arc<RwLock<HashMap<SocketAddr, (CancellationToken, Type)>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

pub async fn close(addr: &SocketAddr) -> WebResult<()> {
    CANCEL_MAP
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
        CANCEL_MAP
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
