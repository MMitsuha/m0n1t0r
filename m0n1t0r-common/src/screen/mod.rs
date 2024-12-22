use crate::Result as AppResult;
use anyhow::{bail, Result};
use capscreen::{
    capturer::{Capturer, Config, Engine},
    frame::Frame,
    util::{self, Permission},
};
use openh264::{encoder::Encoder, formats::YUVSlices};
use rayon::prelude::{
    IndexedParallelIterator as _, IntoParallelRefIterator as _, ParallelIterator as _,
};
use remoc::{
    rch::lr::{self, Receiver, Sender},
    rtc,
};
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::runtime::Runtime;
use yuvutils_rs::{YuvChromaSubsampling, YuvPlanarImageMut, YuvRange, YuvStandardMatrix};

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

fn process_frames(
    capturer: &mut Capturer,
    encoder: &mut Encoder,
    runtime: &Runtime,
    tx: &mut Sender<Vec<u8>>,
) -> Result<()> {
    loop {
        let frame = capturer.get_frame()?;
        let stream = match frame {
            Frame::Rgba8(rgba8) => {
                let mut planar = YuvPlanarImageMut::<u8>::alloc(
                    rgba8.width,
                    rgba8.height,
                    YuvChromaSubsampling::Yuv420,
                );
                yuvutils_rs::rgba_to_yuv420(
                    &mut planar,
                    &rgba8.data,
                    rgba8.row_stride,
                    YuvRange::Limited,
                    YuvStandardMatrix::Bt601,
                )?;
                encoder.encode(&YUVSlices::new(
                    (
                        planar.y_plane.borrow(),
                        planar.u_plane.borrow(),
                        planar.v_plane.borrow(),
                    ),
                    (planar.width as usize, planar.height as usize),
                    (
                        planar.y_stride as usize,
                        planar.u_stride as usize,
                        planar.v_stride as usize,
                    ),
                ))?
            }
            Frame::Bgra8(bgra8) => {
                let mut planar = YuvPlanarImageMut::<u8>::alloc(
                    bgra8.width,
                    bgra8.height,
                    YuvChromaSubsampling::Yuv420,
                );
                yuvutils_rs::bgra_to_yuv420(
                    &mut planar,
                    &bgra8.data,
                    bgra8.row_stride,
                    YuvRange::Limited,
                    YuvStandardMatrix::Bt601,
                )?;
                encoder.encode(&YUVSlices::new(
                    (
                        planar.y_plane.borrow(),
                        planar.u_plane.borrow(),
                        planar.v_plane.borrow(),
                    ),
                    (planar.width as usize, planar.height as usize),
                    (
                        planar.y_stride as usize,
                        planar.u_stride as usize,
                        planar.v_stride as usize,
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
            _ => bail!("Unsupported frame format"),
        };

        runtime.block_on(tx.send(stream.to_vec()))?;
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn record(&self, options: Config) -> AppResult<Receiver<Vec<u8>>> {
        let options = options;
        let (mut tx, remote_rx) = lr::channel();

        thread::spawn(move || {
            let mut capturer = Capturer::new(&options)?;
            let mut encoder = Encoder::new()?;
            let runtime = Runtime::new()?;

            capturer.start()?;
            let _ = process_frames(&mut capturer, &mut encoder, &runtime, &mut tx);
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
