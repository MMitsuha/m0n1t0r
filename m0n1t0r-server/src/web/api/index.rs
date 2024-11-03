use crate::{web::error::Result as WebResult, web::Response, ServerMap};
use actix_web::{get, web, Responder};
use m0n1t0r_common::util;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
struct GetIndexResponse {
    version: String,
}

impl GetIndexResponse {
    fn new() -> Self {
        Self {
            version: util::version::get(),
        }
    }
}

#[get("/")]
pub async fn get(_data: web::Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(web::Json(Response::success(GetIndexResponse::new())?))
}
