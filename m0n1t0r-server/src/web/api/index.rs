use crate::{web::api::Response, ServerMap};
use actix_web::{get, web, HttpResponse, Responder, Result as WebResult};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("/")]
pub async fn get(_data: web::Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    Ok(HttpResponse::Ok().json(Response::success(Value::Null)?))
}
