use crate::web::{Response, Result as WebResult, api::global::proxy::*, error::Error};
use actix_web::{
    Responder, delete, get,
    web::{Json, Path},
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Detail {
    addr: SocketAddr,
    r#type: Type,
}

pub async fn close(addr: &SocketAddr) -> WebResult<()> {
    PROXY_MAP
        .read()
        .await
        .get(addr)
        .ok_or(Error::NotFound)?
        .0
        .cancel();
    Ok(())
}

#[get("")]
pub async fn get() -> WebResult<impl Responder> {
    Ok(Json(Response::success(Detail::new().await)?))
}

#[delete("/{addr}")]
pub async fn delete(addr: Path<SocketAddr>) -> WebResult<impl Responder> {
    Ok(Json(Response::success(close(&addr).await?)?))
}

impl Detail {
    pub async fn new() -> Vec<Self> {
        PROXY_MAP
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
