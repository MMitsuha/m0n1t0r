use crate::Result as AppResult;
use anyhow::{anyhow, bail, Result};
use openh264::{
    encoder::Encoder,
    formats::{BgraSliceU8, RgbSliceU8, RgbaSliceU8, YUVBuffer, YUVSlices},
};
use rayon::prelude::{
    IndexedParallelIterator as _, IntoParallelRefIterator, ParallelIterator as _,
};
use remoc::{
    rch::lr::{self, Receiver},
    rtc,
};
use ring_channel::RingSender;
use scap::{
    capturer::{Area, Capturer, Resolution},
    frame::FrameType,
};
use serde::{Deserialize, Serialize};
use std::{num::NonZero, thread};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Options {
    pub fps: u32,
    pub show_cursor: bool,
    pub show_highlight: bool,
    pub crop_area: Option<Area>,
    pub output_type: FrameType,
    pub output_resolution: Resolution,
}

impl Options {
    pub fn into_scap(self) -> scap::capturer::Options {
        scap::capturer::Options {
            fps: self.fps,
            show_cursor: self.show_cursor,
            show_highlight: self.show_highlight,
            crop_area: self.crop_area,
            output_type: self.output_type,
            output_resolution: self.output_resolution,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Availability {
    pub support: bool,
    pub has_permission: bool,
}

impl Into<bool> for Availability {
    fn into(self) -> bool {
        self.support && self.has_permission
    }
}

fn process_frame(
    recorder: &mut Capturer,
    encoder: &mut Encoder,
    tx: RingSender<Vec<u8>>,
) -> Result<()> {
    loop {
        let frame = recorder.get_next_frame()?;
        let stream = match frame {
            scap::frame::Frame::RGBx(bgrxframe) if bgrxframe.height * bgrxframe.width != 0 => {
                encoder.encode(&YUVBuffer::from_rgb_source(RgbaSliceU8::new(
                    &bgrxframe.data,
                    (bgrxframe.width.try_into()?, bgrxframe.height.try_into()?),
                )))?
            }
            scap::frame::Frame::BGRx(bgrxframe) if bgrxframe.height * bgrxframe.width != 0 => {
                encoder.encode(&YUVBuffer::from_rgb_source(BgraSliceU8::new(
                    &bgrxframe.data,
                    (bgrxframe.width.try_into()?, bgrxframe.height.try_into()?),
                )))?
            }
            scap::frame::Frame::RGB(rgbframe) if rgbframe.height * rgbframe.width != 0 => {
                encoder.encode(&YUVBuffer::from_rgb_source(RgbSliceU8::new(
                    &rgbframe.data,
                    (rgbframe.width.try_into()?, rgbframe.height.try_into()?),
                )))?
            }
            scap::frame::Frame::BGR0(bgr0frame) if bgr0frame.height * bgr0frame.width != 0 => {
                encoder.encode(&YUVBuffer::from_rgb_source(BgraSliceU8::new(
                    &bgr0frame.data,
                    (bgr0frame.width.try_into()?, bgr0frame.height.try_into()?),
                )))?
            }
            scap::frame::Frame::BGRA(bgraframe) if bgraframe.height * bgraframe.width != 0 => {
                encoder.encode(&YUVBuffer::from_rgb_source(BgraSliceU8::new(
                    &bgraframe.data,
                    (bgraframe.width.try_into()?, bgraframe.height.try_into()?),
                )))?
            }
            scap::frame::Frame::YUVFrame(yuvframe) if yuvframe.height * yuvframe.width != 0 => {
                let u = yuvframe
                    .chrominance_bytes
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                let v = yuvframe
                    .chrominance_bytes
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 1)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                encoder.encode(&YUVSlices::new(
                    (&yuvframe.luminance_bytes, &u, &v),
                    (yuvframe.width, yuvframe.height),
                    (
                        yuvframe.luminance_stride,
                        yuvframe.chrominance_stride / 2,
                        yuvframe.chrominance_stride / 2,
                    ),
                ))?
            }
            _ => bail!("unsupported frame type"),
        };

        tx.send(stream.to_vec())?;
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn record(&self, options: Options) -> AppResult<Receiver<Vec<u8>>> {
        let options = options;
        let (mut tx, remote_rx) = lr::channel();
        let (local_tx, local_rx) = ring_channel::ring_channel(
            NonZero::new((options.fps * 2) as usize).ok_or(anyhow!("fps is zero"))?,
        );

        tokio::spawn(async move {
            loop {
                tx.send(local_rx.recv()?).await?;
            }
            #[allow(unreachable_code)]
            Ok::<_, anyhow::Error>(())
        });

        thread::spawn(move || {
            let mut recorder = Capturer::build(options.into_scap())?;
            let mut encoder = Encoder::new()?;

            recorder.start_capture();
            let _ = process_frame(&mut recorder, &mut encoder, local_tx);
            recorder.stop_capture();
            Ok::<_, anyhow::Error>(())
        });
        Ok(remote_rx)
    }

    async fn is_supported(&self) -> AppResult<bool> {
        Ok(scap::is_supported())
    }

    async fn has_permission(&self) -> AppResult<bool> {
        Ok(scap::has_permission())
    }

    async fn request_permission(&self) -> AppResult<bool> {
        Ok(scap::request_permission())
    }

    async fn availability(&self) -> AppResult<Availability> {
        Ok(Availability {
            support: scap::is_supported(),
            has_permission: scap::has_permission(),
        })
    }
}
