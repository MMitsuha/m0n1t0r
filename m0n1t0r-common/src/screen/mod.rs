use crate::Result as AppResult;
use anyhow::anyhow;
use openh264::{
    encoder::Encoder,
    formats::{BgraSliceU8, YUVBuffer},
};
use remoc::{
    rch::lr::{self, Receiver},
    rtc,
};
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

#[rtc::remote]
pub trait Agent: Sync {
    async fn record(&self, options: Options) -> AppResult<Receiver<Vec<u8>>> {
        let mut options = options;
        options.output_type = FrameType::BGRAFrame;
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
            loop {
                let frame = recorder.get_next_frame()?;
                let stream = match frame {
                    scap::frame::Frame::BGRA(bgraframe)
                        if bgraframe.height * bgraframe.width != 0 =>
                    {
                        encoder.encode(&YUVBuffer::from_rgb_source(BgraSliceU8::new(
                            &bgraframe.data,
                            (bgraframe.width.try_into()?, bgraframe.height.try_into()?),
                        )))?
                    }
                    _ => continue,
                };
                local_tx.send(stream.to_vec())?;
            }
            #[allow(unreachable_code)]
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
