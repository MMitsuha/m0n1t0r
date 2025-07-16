pub mod autorun;
pub mod detail;
pub mod environment;
pub mod fs;
pub mod info;
pub mod network;
pub mod notify;
pub mod process;
pub mod proxy;
pub mod qq;
pub mod terminate;
pub mod update;

use crate::{
    ServerMap,
    web::{Response, Result as WebResult},
};
use actix_web::{
    Responder, get,
    web::{Data, Json},
};
use detail::Detail;
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinSet};

#[get("")]
pub async fn get(data: Data<Arc<RwLock<ServerMap>>>) -> WebResult<impl Responder> {
    let lock_map = data.read().await.map.clone();
    let details = Arc::new(RwLock::new(Vec::new()));
    let mut tasks = JoinSet::new();

    lock_map.into_iter().for_each(|(addr, server)| {
        let details = details.clone();
        tasks.spawn(async move {
            let lock_obj = server.read().await;
            let client = lock_obj.client()?;

            details
                .write()
                .await
                .push(Detail::new(&addr, client).await?);
            Ok::<_, anyhow::Error>(())
        });
    });

    tasks
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(Response::success(&*details.read().await)?))
}
