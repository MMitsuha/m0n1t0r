use crate::{Error, Result as AppResult};
use hbb_common::protobuf::Message;
use remoc::{rch::lr, rtc};
use scrap::{
    Capturer, TraitCapturer, VpxEncoderConfig, VpxVideoCodecId,
    codec::{Encoder, EncoderCfg},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::{runtime::Handle, task};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Display {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub is_online: bool,
    pub is_primary: bool,
    pub origin: (i32, i32),
}

impl From<&scrap::Display> for Display {
    fn from(display: &scrap::Display) -> Self {
        Self {
            name: display.name(),
            width: display.width(),
            height: display.height(),
            is_online: display.is_online(),
            is_primary: display.is_primary(),
            origin: display.origin(),
        }
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}x{})", self.name, self.width, self.height)
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn displays(&self) -> AppResult<Vec<Display>> {
        Ok(scrap::Display::all()?
            .into_iter()
            .map(|display| Display::from(&display))
            .collect())
    }

    async fn view(
        &self,
        display: Display,
        quality: f32,
        i444: bool,
    ) -> AppResult<lr::Receiver<Vec<u8>>> {
        let (mut tx, rx) = lr::channel();

        task::spawn_blocking(move || {
            let display = scrap::Display::all()?
                .into_iter()
                .filter(|d| Display::from(d) == display)
                .next()
                .ok_or(Error::NotFound)?;
            let (width, height) = (display.width(), display.height());
            let mut capturer = Capturer::new(display)?;
            let mut encoder = Encoder::new(
                EncoderCfg::VPX(VpxEncoderConfig {
                    width: width as u32,
                    height: height as u32,
                    quality,
                    keyframe_interval: None,
                    codec: VpxVideoCodecId::VP9,
                }),
                i444,
            )?;
            let mut yuv: Vec<u8> = Vec::new();
            let mut mid: Vec<u8> = Vec::new();
            let start = Instant::now();

            loop {
                if let Ok(frame) = capturer.frame(Duration::from_millis(0)) {
                    let ms = (Instant::now() - start).as_millis() as i64;
                    let frame = frame.to(encoder.yuvfmt(), &mut yuv, &mut mid)?;
                    if let Ok(vf) = encoder.encode_to_message(frame, ms) {
                        Handle::current().block_on(tx.send(vf.write_to_bytes()?))?;
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok::<_, Error>(())
        });
        Ok(rx)
    }
}
