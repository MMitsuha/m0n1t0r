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
use core::slice;
use ffmpeg_next::{Packet, Rational, codec, format::Pixel, frame::Video};
use hbb_common::{
    message_proto::{VideoFrame, video_frame::Union::Vp9s},
    protobuf::Message as _,
};
use m0n1t0r_common::rd::Agent as _;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::{ParallelSlice, ParallelSliceMut},
};
use scrap::{
    CodecFormat, GoogleImage, ImageFormat, ImageRgb, ImageTexture, STRIDE_ALIGN, VpxDecoder,
    VpxDecoderConfig, VpxVideoCodecId, codec::Decoder,
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
    kf: Option<usize>,
    format: Option<Format>,
}

#[get("/stream/mpeg1video")]
pub async fn get_mpeg1video(
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
    let mut rx = agent.view(display.clone(), query.quality, query.kf).await?;

    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        let mut decoder = VpxDecoder::new(VpxDecoderConfig {
            codec: VpxVideoCodecId::VP9,
        })?;
        let codec = ffmpeg_next::encoder::find(codec::Id::MPEG1VIDEO)
            .ok_or(ffmpeg_next::Error::EncoderNotFound)?;
        let mut encoder = codec::Context::new_with_codec(codec).encoder().video()?;
        let frame_rate = 25;
        encoder.set_width(display.width as u32);
        encoder.set_height(display.height as u32);
        encoder.set_format(Pixel::YUV420P);
        encoder.set_frame_rate(Some(Rational(frame_rate, 1)));
        encoder.set_time_base(Rational(1, frame_rate));
        encoder.set_max_b_frames(0);
        let mut encoder = encoder.open_as(codec)?;
        let mut i = 0u64;
        let mut video = Video::new(
            encoder.format(),
            display.width as u32,
            display.height as u32,
        );

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
                    if let Some(Vp9s(evfs)) = vf.union {
                        for evf in evfs.frames {
                            for frame in decoder.decode(&evf.data)? {
                                let src_planes = frame.planes();
                                let src_strides = frame.stride();
                                let heights = vec![display.height, display.height / 2, display.height / 2];
                                let dst_strides = vec![video.stride(0), video.stride(1), video.stride(2)];
                                unsafe {
                                    for i in 0..3 {
                                        slice::from_raw_parts(src_planes[i], src_strides[i] as usize * heights[i])
                                            .par_chunks(src_strides[i] as usize)
                                            .zip(video.data_mut(i).par_chunks_mut(dst_strides[i]))
                                            .for_each(|(src, dst)| {
                                                dst.copy_from_slice(&src[..dst_strides[i]]);
                                            });
                                    }
                                }
                                video.set_pts(Some(i as i64));
                                encoder.send_frame(&video)?;
                                let mut encoded = Packet::empty();
                                while encoder.receive_packet(&mut encoded).is_ok() {
                                    session.binary(encoded.data().ok_or(Error::NotFound)?.to_vec()).await?;
                                }
                                i += 1;
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

#[get("/stream/yuv")]
pub async fn get_yuv(
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
    let mut rx = agent.view(display.clone(), query.quality, query.kf).await?;

    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        let mut decoder = VpxDecoder::new(VpxDecoderConfig {
            codec: VpxVideoCodecId::VP9,
        })?;

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
                    if let Some(Vp9s(evfs)) = vf.union {
                        for evf in evfs.frames {
                            for frame in decoder.decode(&evf.data)? {
                                let pixels = display.width * display.height;
                                let heights = vec![display.height, display.height / 2, display.height / 2];
                                let src_planes = frame.planes();
                                let src_strides = frame.stride();
                                let mut buffer = vec![0; pixels * 3 / 2];
                                let dst_strides = vec![display.width, display.width / 2, display.width / 2];
                                let (dst_planes_1, dst_planes_12) = buffer.split_at_mut(pixels);
                                let (dst_planes_2, dst_planes_3) = dst_planes_12.split_at_mut(pixels / 4);
                                let mut dst_planes = vec![dst_planes_1, dst_planes_2, dst_planes_3];
                                unsafe {
                                    for i in 0..3 {
                                        slice::from_raw_parts(src_planes[i], src_strides[i] as usize * heights[i])
                                            .par_chunks(src_strides[i] as usize)
                                            .zip(dst_planes[i].par_chunks_mut(dst_strides[i]))
                                            .for_each(|(src, dst)| {
                                                dst.copy_from_slice(&src[..dst_strides[i]]);
                                            });
                                    }
                                }
                                session.binary(buffer).await?;
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

#[get("/stream/rgb")]
pub async fn get_rgb(
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
    let mut rx = agent.view(display.clone(), query.quality, query.kf).await?;

    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    task::spawn_local(util::handle_websocket(session.clone(), async move {
        let mut decoder = Decoder::new(CodecFormat::VP9, None);
        let mut pixelbuffer = true;
        let mut chroma = None;
        let mut rgb = ImageRgb::new(
            match query.format.ok_or(Error::NotFound)? {
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
                            session.binary(rgb.raw.to_vec()).await?;
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
