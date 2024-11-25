use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get, head, put,
    web::{Data, Json, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::{Message, Session};
use anyhow::{anyhow, Result};
use libsw::{Instant, StopwatchImpl, Sw};
use m0n1t0r_common::{
    client::Client,
    screen::{Agent, Options},
};
use scap::capturer::Resolution;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[derive(Serialize, Deserialize)]
struct FrameDetail {
    fps: f32,
}

#[get("")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    let canceller = lock_obj.get_canceller();
    drop(lock_obj);

    if Into::<bool>::into(agent.availability().await?) == false {
        return Err(Error::UnsupportedError);
    }

    let mut rx = agent
        .record(Options {
            fps: 120,
            show_cursor: true,
            show_highlight: true,
            output_resolution: Resolution::_720p,
            ..Default::default()
        })
        .await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(async move {
        let mut stopwatch = Sw::new_started();

        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                frame = rx.recv() => process_frame(&mut session, frame?.ok_or(anyhow!("no frame received"))?, &mut stopwatch).await?,
                _ = canceller.cancelled() => break,
            }
        }
        session.close(None).await?;
        Ok::<_, anyhow::Error>(())
    });
    Ok(response)
}

async fn process_frame<I: Instant>(
    session: &mut Session,
    frame: Vec<u8>,
    stopwatch: &mut StopwatchImpl<I>,
) -> Result<()> {
    let elapsed = stopwatch.elapsed();
    session
        .text(serde_json::to_string(&FrameDetail {
            fps: 1f32 / elapsed.as_secs_f32(),
        })?)
        .await?;
    session.binary(frame).await?;
    stopwatch.reset_in_place();
    Ok(())
}

#[put("")]
pub async fn put(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    drop(lock_obj);

    let availability = agent.availability().await?;

    if Into::<bool>::into(availability.clone()) == true {
        return Ok(Json(Response::success(())?));
    }

    if availability.support == false {
        return Err(Error::UnsupportedError);
    }

    if agent.request_permission().await? == false {
        return Err(Error::ClientDeniedError);
    }

    Ok(Json(Response::success(())?))
}

#[head("")]
pub async fn head(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFoundError)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    drop(lock_obj);

    Ok(Json(Response::success(agent.availability().await?)?))
}
