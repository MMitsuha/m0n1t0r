use super::{Bgra8, Frame};
use crate::{Error, Result};
use windows_capture::{
    frame::{Frame as RawFrame, FrameBuffer},
    settings::ColorFormat,
};

impl From<FrameBuffer<'_>> for Bgra8 {
    fn from(value: FrameBuffer<'_>) -> Self {
        let buffer = value.as_raw_buffer();
        Self {
            width: value.width(),
            row_stride: value.row_pitch(),
            height: value.height(),
            height_stride: value.depth_pitch(),
            data: buffer.to_vec(),
        }
    }
}

impl TryFrom<&mut RawFrame<'_>> for Frame {
    type Error = Error;

    fn try_from(value: &mut RawFrame<'_>) -> Result<Self> {
        let color_format = value.get_color_format();
        let buffer = value
            .buffer()
            .map_err(|e| Error::ParseFrameFailed(e.into()))?;
        Ok(match color_format {
            ColorFormat::Bgra8 => Frame::Bgra8(buffer.into()),
            _ => todo!(),
        })
    }
}
