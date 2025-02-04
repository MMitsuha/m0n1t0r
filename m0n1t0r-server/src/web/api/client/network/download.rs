use crate::{
    web::{api::client::network, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    post,
    web::{Data, Form, Json, Path},
    Responder,
};
use m0n1t0r_common::network::Agent as _;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

#[derive(Deserialize)]
struct DownloadForm {
    url: Url,
    path: PathBuf,
}

#[post("/download")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    form: Form<DownloadForm>,
) -> WebResult<impl Responder> {
    let form = form.into_inner();
    let (agent, _) = network::get_agent(data, &addr).await?;

    Ok(Json(Response::success(
        agent.download(form.url, form.path).await?,
    )?))
}
