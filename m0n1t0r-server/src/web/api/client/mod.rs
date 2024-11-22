pub mod client;
pub mod fs;
pub mod info;
pub mod network;
pub mod process;
pub mod proxy;
pub mod screen;

use crate::{
    web::{Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use m0n1t0r_common::client::Client as _;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
struct Get {
    count: usize,
    clients: Vec<client::Get>,
}

impl Get {
    async fn new(server_map: Arc<RwLock<ServerMap>>) -> WebResult<Self> {
        let lock_map = server_map.read().await;
        let mut clients = Vec::new();

        for (addr, server) in lock_map.iter() {
            let lock_obj = server.read().await;
            let client = lock_obj.get_client()?;

            clients.push(
                client::Get::new(
                    addr,
                    client.version().await?,
                    client.target_platform().await?,
                )
                .await?,
            );
        }
        Ok(Self {
            count: lock_map.len(),
            clients,
        })
    }
}

#[get("")]
pub async fn get(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(Json(Response::success(Get::new((**data).clone()).await?)?))
}
