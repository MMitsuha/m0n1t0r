use crate::web::{Response, Result as WebResult, api::global::proxy::*, error::Error};
use actix_web::{
    Responder, delete, get,
    web::{Json, Path},
};
use slotmap::{DefaultKey, KeyData};

pub async fn close(key: DefaultKey) -> WebResult<()> {
    PROXY_MAP
        .read()
        .await
        .get(key)
        .ok_or(Error::NotFound)?
        .canceller
        .cancel();
    Ok(())
}

#[get("")]
pub async fn get() -> WebResult<impl Responder> {
    Ok(Json(Response::success(PROXY_MAP.read().await.as_vec())?))
}

#[delete("/{key}")]
pub async fn delete(key: Path<u64>) -> WebResult<impl Responder> {
    Ok(Json(Response::success(
        close(KeyData::from_ffi(*key).into()).await?,
    )?))
}
