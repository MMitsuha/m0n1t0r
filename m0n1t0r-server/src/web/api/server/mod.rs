pub mod notify;
pub mod proxy;

use crate::web::{Response, Result as WebResult};
use actix_web::{Responder, get, web::Json};
use m0n1t0r_common::util;
use serde::Serialize;

#[derive(Serialize)]
struct Detail {
    version: String,
    build_time: String,
    commit_hash: String,
}

impl Detail {
    fn new() -> Self {
        Self {
            version: util::version::version().into(),
            build_time: util::version::build_time().into(),
            commit_hash: util::version::commit_hash().into(),
        }
    }
}

#[get("")]
pub async fn get() -> WebResult<impl Responder> {
    Ok(Json(Response::success(Detail::new())?))
}
