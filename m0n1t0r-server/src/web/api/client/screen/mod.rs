use crate::{
    web::{self, Error, Response, Result as WebResult},
    ServerMap,
};
use actix_web::{
    get, put,
    web::{Data, Json, Path, Payload},
    HttpRequest, Responder,
};
use actix_ws::{Message, Session};
use anyhow::{anyhow, Result};
use capscreen::capturer::Config;
use m0n1t0r_common::{client::Client, screen::Agent};
use openh264::{decoder::Decoder, formats::YUVSource};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, ParallelIterator as _, ParallelSlice,
    ParallelSliceMut,
};
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
    #[serde(rename = "yuv2")]
    Yuy2,
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
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    let canceller = lock_obj.get_canceller();
    drop(lock_obj);

    if Into::<bool>::into(agent.availability().await?) == false {
        return Err(Error::Unsupported);
    }

    let mut rx = agent.record(Config::main(120)).await?;
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    stream = stream.max_frame_size(134217728);
    task::spawn_local(web::handle_websocket(session.clone(), async move {
        let mut decoder = Decoder::new()?;

        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                frame = rx.recv() => match r#type {
                    Type::Raw => process_raw(&mut session, frame?.ok_or(anyhow!("no frame received"))?).await?,
                    Type::Yuy2 => process_yuy2(&mut session, frame?.ok_or(anyhow!("no frame received"))?, &mut decoder).await?,
                    Type::Nv12 => process_nv12(&mut session, frame?.ok_or(anyhow!("no frame received"))?, &mut decoder).await?,
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

async fn process_yuy2(session: &mut Session, frame: Vec<u8>, decoder: &mut Decoder) -> Result<()> {
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

async fn process_nv12(session: &mut Session, frame: Vec<u8>, decoder: &mut Decoder) -> Result<()> {
    if let Some(frame) = decoder.decode(&frame)? {
        let (width, height) = frame.dimensions();
        let (y_stride, uv_stride, _) = frame.strides();
        let mut buffer = vec![0u8; width * height * 3 / 2];
        let (buffer_y, buffer_uv) = buffer.split_at_mut(width * height);

        frame
            .y()
            .par_chunks(y_stride)
            .map(|y_row| &y_row[..width])
            .zip(buffer_y.par_chunks_mut(width))
            .for_each(|(src, dst)| {
                dst.copy_from_slice(src);
            });
        frame
            .u()
            .par_chunks(uv_stride)
            .map(|u_row| &u_row[..width / 2])
            .zip(
                frame
                    .v()
                    .par_chunks(uv_stride)
                    .map(|v_row| &v_row[..width / 2]),
            )
            .zip(buffer_uv.par_chunks_mut(width))
            .for_each(|((u_row, v_row), dst_row)| {
                u_row
                    .into_par_iter()
                    .zip(v_row.into_par_iter())
                    .zip(dst_row.par_chunks_mut(2))
                    .for_each(|((u, v), dst)| dst.copy_from_slice(&[*u, *v]))
            });
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
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    drop(lock_obj);

    let availability = agent.availability().await?;

    if Into::<bool>::into(availability.clone()) == true {
        return Ok(Json(Response::success(())?));
    }

    if availability.support == false {
        return Err(Error::Unsupported);
    }

    if agent.request_permission().await? == false {
        return Err(Error::ClientDeniedRequest);
    }

    Ok(Json(Response::success(())?))
}

#[get("")]
pub async fn head(
    data: Data<Arc<RwLock<ServerMap>>>,
    path: Path<SocketAddr>,
) -> WebResult<impl Responder> {
    let addr = path.into_inner();
    let lock_map = &data.read().await.map;
    let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let agent = client.get_screen_agent().await?;
    drop(lock_obj);

    Ok(Json(Response::success(agent.availability().await?)?))
}
