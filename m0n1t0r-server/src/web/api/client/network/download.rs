use crate::{
    ServerMap,
    web::{Response, Result as WebResult, api::client::network},
};
use actix_web::{
    Responder, post,
    web::{Data, Form, Json, Path},
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
    Form(form): Form<DownloadForm>,
) -> WebResult<impl Responder> {
    let (agent, _) = network::agent(data, &addr).await?;

    Ok(Json(Response::success(
        agent.download(form.url, form.path).await?,
    )?))
}
