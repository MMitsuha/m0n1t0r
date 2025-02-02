use crate::{
    web::{api::client::proxy::socks5, Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    delete, get, put,
    web::{Bytes, Data, Json, Path},
    HttpResponse, Responder,
};
use m0n1t0r_common::{client::Client, fs::Agent as _};
use qqkey::QQ;
use reqwest::Proxy;
use scopeguard::defer;
use serde::{Deserialize, Serialize};
use socks5_impl::server::auth::NoAuth;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::DropGuard;

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let auth = Arc::new(NoAuth::default());
    let (addr, _, canceller) =
        socks5::open_internal(data, &addr, "127.0.0.1:0".parse().unwrap(), auth).await?;
    let _guard = canceller.drop_guard();
    let qq = QQ::new_with_proxy(Proxy::all(format!("socks5://{}", addr)).unwrap()).await?;
    let response = Response::success(qq.get_logged_qq_info().await?)?;

    Ok(Json(response))
}
