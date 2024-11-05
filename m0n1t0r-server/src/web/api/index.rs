use crate::{
    web::{Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use m0n1t0r_common::util;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
struct Get {
    version: String,
}

impl Get {
    fn new() -> Self {
        Self {
            version: util::version::get(),
        }
    }
}

#[get("")]
pub async fn get(_data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(Json(Response::success(Get::new())?))
}
