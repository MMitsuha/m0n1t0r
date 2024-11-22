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
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("")]
pub async fn get(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    let lock_map = data.read().await;
    let mut details = Vec::new();

    for (addr, server) in lock_map.iter() {
        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        details.push(
            client::Detail::new(
                addr,
                client.version().await?,
                client.target_platform().await?,
                client.system_info().await?,
            )
            .await?,
        );
    }

    Ok(Json(Response::success(details)?))
}
