use super::*;
use crate::{Error, Result};
use core_foundation::base::TCFType;
use screencapturekit::output::{
    sc_stream_frame_info::{SCFrameStatus, SCStreamFrameInfo},
    CMSampleBuffer, CVPixelBufferRef, LockTrait,
};
use std::slice;

impl TryFrom<CMSampleBuffer> for Nv12 {
    type Error = Error;

    fn try_from(value: CMSampleBuffer) -> Result<Self> {
        extern "C" {
            fn CVPixelBufferGetBaseAddressOfPlane(
                pixelBuffer: CVPixelBufferRef,
                planeIndex: libc::size_t,
            ) -> *mut libc::c_void;

            fn CVPixelBufferGetBytesPerRowOfPlane(
                pixelBuffer: CVPixelBufferRef,
                planeIndex: libc::size_t,
            ) -> libc::size_t;
        }

        let pixels = value
            .get_pixel_buffer()
            .map_err(|e| Error::ParseFrameFailed(e.into()))?;
        let width = pixels.get_width();
        let height = pixels.get_height();

        if pixels.is_planar() == false {
            return Err(Error::ParseFrameFailed(
                "The pixel buffer is not planar".into(),
            ));
        }

        let _lock = pixels
            .lock()
            .map_err(|e| Error::ParseFrameFailed(e.into()))?;

        let y_stride =
            unsafe { CVPixelBufferGetBytesPerRowOfPlane(pixels.as_concrete_TypeRef(), 0) };
        let y = unsafe {
            slice::from_raw_parts(
                CVPixelBufferGetBaseAddressOfPlane(pixels.as_concrete_TypeRef(), 0) as *const u8,
                height as usize * y_stride,
            )
        }
        .to_vec();
        let uv_stride =
            unsafe { CVPixelBufferGetBytesPerRowOfPlane(pixels.as_concrete_TypeRef(), 1) };
        let uv = unsafe {
            slice::from_raw_parts(
                CVPixelBufferGetBaseAddressOfPlane(pixels.as_concrete_TypeRef(), 1) as *const u8,
                height as usize * uv_stride / 2,
            )
        }
        .to_vec();
        Ok(Self {
            y,
            uv,
            y_stride,
            uv_stride,
            width,
            height,
        })
    }
}

impl TryFrom<CMSampleBuffer> for Frame {
    type Error = Error;

    fn try_from(buffer: CMSampleBuffer) -> Result<Self> {
        let status = SCStreamFrameInfo::from_sample_buffer(&buffer)
            .map_err(|e| Error::ParseFrameFailed(e.into()))?;

        match status.status() {
            SCFrameStatus::Complete | SCFrameStatus::Started => {
                Ok(Frame::Nv12(Nv12::try_from(buffer)?))
            }
            SCFrameStatus::Stopped => Err(Error::FrameStopped),
            _ => Ok(Self::Empty),
        }
    }
}
