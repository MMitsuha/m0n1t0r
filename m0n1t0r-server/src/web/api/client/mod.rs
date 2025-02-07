pub mod detail;
pub mod environment;
pub mod fs;
pub mod info;
pub mod network;
pub mod notify;
pub mod process;
pub mod proxy;
pub mod qq;
pub mod terminate;
pub mod update;

use crate::{
    web::{Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use detail::Detail;
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("")]
pub async fn get(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    let lock_map = &data.read().await.map;
    let mut details = Vec::new();

    // TODO: Parallelize this
    for (addr, server) in lock_map.iter() {
        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        details.push(Detail::new(addr, client).await?);
    }
    Ok(Json(Response::success(details)?))
}
