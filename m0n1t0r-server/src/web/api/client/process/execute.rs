use crate::{
    ServerMap,
    web::{
        Response, Result as WebResult,
        api::client::process::{self, CommandForm, Execute},
    },
};
use actix_web::{
    Responder, post,
    web::{Data, Form, Json, Path},
};
use m0n1t0r_common::process::Agent as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[post("/execute")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    Form(form): Form<CommandForm>,
) -> WebResult<impl Responder> {
    let (agent, _) = process::agent(data, &addr).await?;

    let mut command = shell_words::split(&form.command)?;
    let program = command.remove(0);

    match form.option {
        Execute::Blocked => Ok(Json(Response::success(
            agent.execute(program, command).await?,
        )?)),
        Execute::Detached => Ok(Json(Response::success(
            agent.execute_detached(program, command).await?,
        )?)),
    }
}
