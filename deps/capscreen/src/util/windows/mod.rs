use crate::{Error, Result};
use windows_capture::{graphics_capture_api::GraphicsCaptureApi, monitor::Monitor};

pub struct Screen {
    pub id: usize,
    pub width: u32,
    pub height: u32,
}

impl super::Display for Screen {
    fn list() -> Result<Vec<Self>> {
        let monitors = Monitor::enumerate().map_err(|e| Error::ListScreenFailed(e.into()))?;
        let mut screens = Vec::with_capacity(monitors.len());

        for monitor in monitors {
            screens.push(Self {
                id: monitor
                    .index()
                    .map_err(|e| Error::ListScreenFailed(e.into()))?,
                width: monitor
                    .width()
                    .map_err(|e| Error::ListScreenFailed(e.into()))?,
                height: monitor
                    .height()
                    .map_err(|e| Error::ListScreenFailed(e.into()))?,
            });
        }
        Ok(screens)
    }

    fn main() -> Result<Self>
    where
        Self: Sized,
    {
        let monitor = Monitor::primary().map_err(|e| Error::NoDisplayFound(e.into()))?;
        Ok(Self {
            id: monitor
                .index()
                .map_err(|e| Error::NoDisplayFound(e.into()))?,
            width: monitor
                .width()
                .map_err(|e| Error::NoDisplayFound(e.into()))?,
            height: monitor
                .height()
                .map_err(|e| Error::NoDisplayFound(e.into()))?,
        })
    }
}

pub struct Availability;

impl super::Permission for Availability {
    fn has_permission() -> bool {
        true
    }

    fn request_permission() -> bool {
        true
    }

    fn is_supported() -> Result<bool> {
        Ok(GraphicsCaptureApi::is_supported()
            .map_err(|e| Error::AvailabilityDetectionFailed(e.into()))?)
    }
}
