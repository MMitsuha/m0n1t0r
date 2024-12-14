use crate::{error::Result, Error};
use core_graphics_helmer_fork::{access::ScreenCaptureAccess, display::CGDisplay};
use core_graphics_types::geometry::{CGPoint, CGRect, CGSize};
use screencapturekit::shareable_content::{SCDisplay, SCShareableContent};
use sysinfo::System;
use version_compare::Version;

pub struct Screen {
    pub id: usize,
    pub width: u32,
    pub height: u32,
}

impl From<&SCDisplay> for Screen {
    fn from(display: &SCDisplay) -> Self {
        Self {
            id: display.display_id() as usize,
            width: display.width(),
            height: display.height(),
        }
    }
}

impl Screen {
    pub fn get_full_screen_rect(&self) -> CGRect {
        CGRect {
            origin: CGPoint { x: 0.0, y: 0.0 },
            size: CGSize {
                width: self.width as f64,
                height: self.height as f64,
            },
        }
    }

    pub fn from_display_id(id: usize) -> Result<Self> {
        Ok(SCShareableContent::get()
            .map_err(|e| Error::ListScreenFailed(e.into()))?
            .displays()
            .into_iter()
            .find(|d| d.display_id() as usize == id)
            .map(|d| (&d).into())
            .ok_or(Error::NoDisplayFound)?)
    }

    pub fn as_sc_display(&self) -> Result<SCDisplay> {
        Ok(SCShareableContent::get()
            .map_err(|e| Error::ListScreenFailed(e.into()))?
            .displays()
            .into_iter()
            .find(|d| d.display_id() as usize == self.id)
            .ok_or(Error::NoDisplayFound)?)
    }
}

impl super::Display for Screen {
    fn list() -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        Ok(SCShareableContent::get()
            .map_err(|e| Error::ListScreenFailed(e.into()))?
            .displays()
            .iter()
            .map(|d| d.into())
            .collect())
    }

    fn main() -> Result<Self>
    where
        Self: Sized,
    {
        let screen = CGDisplay::main();
        let mode = screen.display_mode().ok_or(Error::NoDisplayFound)?;
        Ok(Self {
            id: screen.id as usize,
            width: mode.width() as u32,
            height: mode.height() as u32,
        })
    }
}

pub struct Availability;

impl super::Permission for Availability {
    fn has_permission() -> bool {
        ScreenCaptureAccess.preflight()
    }

    fn request_permission() -> bool {
        ScreenCaptureAccess.request()
    }

    fn is_supported() -> Result<bool> {
        let version = System::os_version()
            .ok_or(Error::AvailabilityDetectionFailed("no os version".into()))?;
        let semver = Version::from(&version).ok_or(Error::NotSemVer)?;
        let min_version = Version::from("12.3").unwrap();
        Ok(semver >= min_version)
    }
}
