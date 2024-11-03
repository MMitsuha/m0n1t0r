use crate::{web::api::Response, ServerMap};
use actix_web::{get, web, HttpResponse, Responder, Result as WebResult};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize)]
struct Count {
    count: usize,
}

impl Count {
    fn new(count: usize) -> Self {
        Self { count }
    }
}

#[get("/client/count")]
pub async fn get(data: web::Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(HttpResponse::Ok().json(Response::success(Count::new(data.read().await.len()))?))
}
