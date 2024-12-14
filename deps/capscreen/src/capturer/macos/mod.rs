use crate::{
    frame::Frame,
    util::{macos::Screen, Display},
    Error, Result,
};
use ring_channel::{RingReceiver, RingSender};
use screencapturekit::{
    output::CMSampleBuffer,
    stream::{
        configuration::{pixel_format::PixelFormat, SCStreamConfiguration},
        content_filter::SCContentFilter,
        output_trait::SCStreamOutputTrait,
        output_type::SCStreamOutputType,
        SCStream,
    },
};
use std::num::NonZeroUsize;

use super::Config;

struct InternalEngine {
    tx: RingSender<CMSampleBuffer>,
}

impl SCStreamOutputTrait for InternalEngine {
    fn did_output_sample_buffer(&self, sample_buffer: CMSampleBuffer, of_type: SCStreamOutputType) {
        assert_eq!(of_type, SCStreamOutputType::Screen);
        println!("Output");

        let _ = self.tx.send(sample_buffer).unwrap();
    }
}

impl InternalEngine {
    pub fn new(tx: RingSender<CMSampleBuffer>) -> Self {
        Self { tx }
    }
}

pub struct Capturer {
    stream: SCStream,
    rx: RingReceiver<CMSampleBuffer>,
}

impl super::Engine for Capturer {
    fn new(config: &Config) -> Result<Self>
    where
        Self: Sized,
    {
        let screen = match config.display {
            Some(id) => Screen::from_display_id(id),
            None => Screen::main(),
        }?;
        let width = config.width.unwrap_or(screen.width);
        let height = config.height.unwrap_or(screen.height);
        let filter =
            SCContentFilter::new().with_display_excluding_windows(&screen.as_sc_display()?, &[]);
        let config = SCStreamConfiguration::new()
            .set_width(width)
            .map_err(|e| Error::EditConfigFailed(e.into()))?
            .set_height(height)
            .map_err(|e| Error::EditConfigFailed(e.into()))?
            .set_source_rect(screen.get_full_screen_rect())
            .map_err(|e| Error::EditConfigFailed(e.into()))?
            .set_pixel_format(PixelFormat::YCbCr_420v)
            .map_err(|e| Error::EditConfigFailed(e.into()))?;
        let mut stream = SCStream::new(&filter, &config);
        let (tx, rx) = ring_channel::ring_channel(NonZeroUsize::new(120).unwrap()); // TODO fix this

        stream.add_output_handler(InternalEngine::new(tx), SCStreamOutputType::Screen);

        Ok(Self { stream, rx })
    }

    fn start(&mut self) -> Result<()> {
        self.stream
            .start_capture()
            .map_err(|e| Error::EditConfigFailed(e.into()))
    }

    fn stop(&mut self) -> Result<()> {
        self.stream
            .stop_capture()
            .map_err(|e| Error::EditConfigFailed(e.into()))
    }

    fn get_frame(&mut self) -> Result<Frame> {
        Ok(self.rx.recv()?.try_into()?)
    }
}
