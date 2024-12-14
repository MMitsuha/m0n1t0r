use crate::{frame::Frame, Error, Result};
use internal::*;
use ring_channel::{RingReceiver, RingSender};
use std::num::NonZeroUsize;
use windows_capture::{
    capture::{CaptureControl, GraphicsCaptureApiHandler},
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

use super::Config;

type Parameter = RingSender<Frame>;

pub struct Capturer {
    engine: Option<CaptureControl<InternalEngine, Error>>,
    setting: Settings<Parameter, Monitor>,
    rx: RingReceiver<Frame>,
}

impl super::Engine for Capturer {
    fn new(config: &Config) -> Result<Self>
    where
        Self: Sized,
    {
        let screen = match config.display {
            Some(index) => Monitor::from_index(index),
            None => Monitor::primary(),
        }
        .map_err(|e| Error::NoDisplayFound(e.into()))?;
        let (tx, rx) = ring_channel::ring_channel(NonZeroUsize::new(120).unwrap());
        let setting = Settings::new(
            screen,
            CursorCaptureSettings::WithCursor,
            DrawBorderSettings::WithoutBorder,
            ColorFormat::Bgra8,
            tx,
        );
        Ok(Self {
            engine: None,
            setting,
            rx,
        })
    }

    fn start(&mut self) -> Result<()> {
        self.engine = Some(
            InternalEngine::start_free_threaded(self.setting.clone())
                .map_err(|e| Error::CaptureStartFailed(e.into()))?,
        );
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.engine
            .take()
            .ok_or(Error::AlreadyStopped)?
            .stop()
            .map_err(|e| Error::CaptureStopFailed(e.into()))?;
        Ok(())
    }

    fn get_frame(&mut self) -> Result<Frame> {
        Ok(self.rx.recv()?)
    }
}

mod internal {
    use super::Parameter;
    use crate::{frame::Frame, Error};
    use ring_channel::RingSender;
    use windows_capture::{
        capture::{Context, GraphicsCaptureApiHandler},
        frame::Frame as RawFrame,
        graphics_capture_api::InternalCaptureControl,
    };

    pub(crate) struct InternalEngine {
        tx: RingSender<Frame>,
    }

    impl GraphicsCaptureApiHandler for InternalEngine {
        type Flags = Parameter;
        type Error = Error;

        fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
            Ok(Self { tx: ctx.flags })
        }

        fn on_frame_arrived(
            &mut self,
            frame: &mut RawFrame,
            capture_control: InternalCaptureControl,
        ) -> Result<(), Self::Error> {
            self.tx.send(frame.try_into()?).map_err(|e| {
                capture_control.stop();
                e
            })?;
            Ok(())
        }
    }
}
