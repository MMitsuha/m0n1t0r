use crate::Result as AppResult;
use anyhow::{anyhow, Result};
use capscreen::{
    capturer::{Capturer, Config, Engine},
    frame::Frame,
    util::{self, Permission},
};
use openh264::{encoder::Encoder, formats::YUVSlices};
use rayon::prelude::{
    IndexedParallelIterator as _, IntoParallelRefIterator, ParallelIterator as _,
};
use remoc::{
    rch::lr::{self, Receiver},
    rtc,
};
use ring_channel::RingSender;
use serde::{Deserialize, Serialize};
use std::{num::NonZero, thread};
use yuvutils_rs::{
    bgra_to_yuv_nv12, YuvBiPlanarImageMut, YuvChromaSubsampling, YuvRange, YuvStandardMatrix,
};

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
    capturer: &mut Capturer,
    encoder: &mut Encoder,
    tx: RingSender<Vec<u8>>,
) -> Result<()> {
    loop {
        let frame = capturer.get_frame()?;
        let stream = match frame {
            Frame::Bgra8(bgra8) => {
                let mut planar = YuvBiPlanarImageMut::<u8>::alloc(
                    bgra8.width,
                    bgra8.height,
                    YuvChromaSubsampling::Yuv420,
                );
                bgra_to_yuv_nv12(
                    &mut planar,
                    &bgra8.data,
                    bgra8.row_stride,
                    YuvRange::Limited,
                    YuvStandardMatrix::Bt601,
                )?;
                let u = planar
                    .uv_plane
                    .borrow()
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                let v = planar
                    .uv_plane
                    .borrow()
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 1)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                encoder.encode(&YUVSlices::new(
                    (planar.y_plane.borrow(), &u, &v),
                    (planar.width as usize, planar.height as usize),
                    (
                        planar.y_stride as usize,
                        planar.uv_stride as usize / 2,
                        planar.uv_stride as usize / 2,
                    ),
                ))?
            }
            Frame::Nv12(nv12) => {
                let u = nv12
                    .uv
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                let v = nv12
                    .uv
                    .par_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 1)
                    .map(|(_, byte)| *byte)
                    .collect::<Vec<_>>();
                encoder.encode(&YUVSlices::new(
                    (&nv12.y, &u, &v),
                    (nv12.width as usize, nv12.height as usize),
                    (nv12.y_stride, nv12.uv_stride / 2, nv12.uv_stride / 2),
                ))?
            }
            Frame::Empty => continue,
        };

        tx.send(stream.to_vec())?;
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn record(&self, options: Config) -> AppResult<Receiver<Vec<u8>>> {
        let options = options;
        let (mut tx, remote_rx) = lr::channel();
        let (local_tx, local_rx) = ring_channel::ring_channel(
            NonZero::new(options.buffer_size).ok_or(anyhow!("buffer size is zero"))?,
        );

        tokio::spawn(async move {
            loop {
                tx.send(local_rx.recv()?).await?;
            }
            #[allow(unreachable_code)]
            Ok::<_, anyhow::Error>(())
        });

        thread::spawn(move || {
            let mut capturer = Capturer::new(&options)?;
            let mut encoder = Encoder::new()?;

            capturer.start()?;
            let _ = process_frame(&mut capturer, &mut encoder, local_tx);
            capturer.stop()?;
            Ok::<_, anyhow::Error>(())
        });
        Ok(remote_rx)
    }

    async fn is_supported(&self) -> AppResult<bool> {
        Ok(util::Availability::is_supported()?)
    }

    async fn has_permission(&self) -> AppResult<bool> {
        Ok(util::Availability::has_permission())
    }

    async fn request_permission(&self) -> AppResult<bool> {
        Ok(util::Availability::request_permission())
    }

    async fn availability(&self) -> AppResult<Availability> {
        Ok(Availability {
            support: util::Availability::is_supported()?,
            has_permission: util::Availability::has_permission(),
        })
    }
}
