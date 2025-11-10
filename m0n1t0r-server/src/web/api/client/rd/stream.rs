use crate::{
    ServerMap,
    web::{Error, Result as WebResult, api::client::rd, util},
};
use actix_web::{
    HttpRequest, Responder, get,
    web::{Data, Path, Payload, Query},
};
use actix_ws::Message;
use anyhow::anyhow;
use hbb_common::{message_proto::VideoFrame, protobuf::Message as _};
use m0n1t0r_common::rd::Agent as _;
use scrap::{
    CodecFormat, GoogleImage, Image, ImageFormat, ImageRgb, ImageTexture, STRIDE_ALIGN,
    codec::Decoder,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock, task};

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Format {
    Raw,
    ABGR,
    ARGB,
}

#[derive(Deserialize)]
struct RdQuery {
    display: usize,
    quality: f32,
    format: Format,
    i444: bool,
}

#[get("/stream")]
pub async fn get(
    data: Data<Arc<RwLock<ServerMap>>>,
    addr: Path<SocketAddr>,
    query: Query<RdQuery>,
    req: HttpRequest,
    body: Payload,
) -> WebResult<impl Responder> {
    let query = query.into_inner();
    let (agent, canceller) = rd::agent(data, &addr).await?;

    let display = agent
        .displays()
        .await?
        .into_iter()
        .nth(query.display)
        .ok_or(Error::NotFound)?;
    let mut rx = agent
        .view(display.clone(), query.quality, query.i444)
        .await?;

    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        let mut decoder = Decoder::new(CodecFormat::VP9, None);
        let mut pixelbuffer = true;
        let mut chroma = None;
        let mut rgb = ImageRgb::new(
            match query.format {
                Format::Raw => ImageFormat::Raw,
                Format::ABGR => ImageFormat::ABGR,
                Format::ARGB => ImageFormat::ARGB,
            },
            STRIDE_ALIGN,
        );
        let mut texture = ImageTexture::default();

        loop {
            select! {
                Some(msg) = stream.recv() => match msg? {
                    Message::Ping(bytes) => session.pong(&bytes).await?,
                    Message::Close(_) => break,
                    _ => {}
                },
                received = rx.recv() => {
                    let vf =
                        VideoFrame::parse_from_bytes(received?.ok_or(anyhow!("channel closed"))?.as_slice())?;
                    if let Some(frame) = vf.union {
                        decoder.handle_video_frame(
                            &frame,
                            &mut rgb,
                            &mut texture,
                            &mut pixelbuffer,
                            &mut chroma,
                        )?;

                        if pixelbuffer {
                            for row in rgb.raw.chunks(<Image as GoogleImage>::get_bytes_per_row(
                                rgb.w, rgb.fmt, rgb.align,
                            )) {
                                session.binary(row.to_vec()).await?;
                            }
                        }
                    }
                },
                _ = canceller.cancelled() => break,
            }
        }
        Ok::<_, anyhow::Error>(())
    }));

    Ok(response)
}
