pub mod info;

use crate::{
    web::error::{Error, Result as WebResult},
    web::Response,
    ServerMap,
};
use actix_web::{get, web, Responder};
use info::GetInfoResponse;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
struct GetClientResponse {
    count: usize,
    clients: Vec<GetInfoResponse>,
}

impl GetClientResponse {
    async fn new(server_map: Arc<RwLock<ServerMap>>) -> WebResult<Self> {
        let lock = server_map.read().await;
        let mut clients = Vec::new();

        for (_, server) in lock.iter() {
            clients.push(
                GetInfoResponse::new(server.clone())
                    .await
                    .map_err(|_| Error::Unknown)?,
            );
        }
        Ok(Self {
            count: lock.len(),
            clients,
        })
    }
}

#[get("/client")]
pub async fn get(data: web::Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(web::Json(Response::success(
        GetClientResponse::new((**data).clone()).await?,
    )?))
}
