use crate::{
    web::{
        api::client::process::{self, CommandForm},
        Response, Result as WebResult,
    },
    ServerMap,
};
use actix_web::{
    post,
    web::{Data, Form, Json, Path},
    Responder,
};
use m0n1t0r_common::process::Agent as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[post("/execute")]
pub async fn post(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    form: Form<CommandForm>,
) -> WebResult<impl Responder> {
    let (agent, _) = process::get_agent(data, &addr).await?;

    let mut command = shell_words::split(&form.command)?;
    let program = command.remove(0);

    Ok(Json(Response::success(
        agent.execute(program, command).await?,
    )?))
}

pub mod detached {
    use super::*;

    #[post("/execute/detached")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        form: Form<CommandForm>,
    ) -> WebResult<impl Responder> {
        let (agent, _) = process::get_agent(data, &addr).await?;

        let mut command = shell_words::split(&form.command)?;
        let program = command.remove(0);

        Ok(Json(Response::success(
            agent.execute_detached(program, command).await?,
        )?))
    }
}
