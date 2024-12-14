use crate::{
    web::{self, Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get, head, put,
    web::{Data, Json, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::{Message, Session};
use anyhow::{anyhow, Result};
use capscreen::capturer::Config;
use libsw::Sw;
use m0n1t0r_common::{client::Client, screen::Agent};
use openh264::{decoder::Decoder, formats::YUVSource};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[derive(Serialize, Deserialize)]
struct FrameDetail {
    fps: f32,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum Type {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "nv12")]
    Nv12,
}

#[get("/{type}")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<(SocketAddr, Type)>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let (addr, r#type) = path.into_inner();
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

    let mut rx = agent.record(Config::main(120)).await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(web::handle_websocket(session.clone(), async move {
        let mut stopwatch = Sw::new_started();
        let mut decoder = Decoder::new()?;

        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                frame = rx.recv() => {
                    let elapsed = stopwatch.elapsed();
                    session
                        .text(serde_json::to_string(&FrameDetail {
                            fps: 1f32 / elapsed.as_secs_f32(),
                        })?)
                        .await?;
                    match r#type {
                        Type::Raw => process_raw(&mut session, frame?.ok_or(anyhow!("no frame received"))?).await?,
                       Type::Nv12 => process_nv12(&mut session, frame?.ok_or(anyhow!("no frame received"))?, &mut decoder).await?,
                    }
                    stopwatch.reset_in_place();
                },
                _ = canceller.cancelled() => break,
            }
        }
        Ok::<_, anyhow::Error>(())
    }));
    Ok(response)
}

async fn process_raw(session: &mut Session, frame: Vec<u8>) -> Result<()> {
    session.binary(frame).await?;
    Ok(())
}

async fn process_nv12(session: &mut Session, frame: Vec<u8>, decoder: &mut Decoder) -> Result<()> {
    if let Some(frame) = decoder.decode(&frame)? {
        let (width, height) = frame.dimensions();
        let (y_stride, uv_stride, _) = frame.strides();
        let mut buffer = Vec::with_capacity(width * height * 2);
        for y in 0..height {
            for x in (0..width).step_by(2) {
                let y1_index = y * y_stride + x;
                let y2_index = y * y_stride + x + 1;
                let uv_index = (y / 2) * uv_stride + (x / 2);
                let y1 = frame.y()[y1_index];
                let y2 = frame.y()[y2_index];
                let u = frame.u()[uv_index];
                let v = frame.v()[uv_index];
                buffer.extend_from_slice(&[y1, u, y2, v]);
            }
        }
        session.binary(buffer).await?;
    }
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
