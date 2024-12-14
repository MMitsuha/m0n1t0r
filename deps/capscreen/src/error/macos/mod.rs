use super::*;

impl From<core_foundation::error::CFError> for Detail {
    fn from(error: core_foundation::error::CFError) -> Self {
        Self {
            code: Some(error.code()),
            message: error.to_string(),
        }
    }
}

impl From<core_media_rs::cm_sample_buffer::error::CMSampleBufferError> for Detail {
    fn from(error: core_media_rs::cm_sample_buffer::error::CMSampleBufferError) -> Self {
        Self {
            code: None,
            message: error.to_string(),
        }
    }
}

impl From<core_video_rs::cv_pixel_buffer::error::CVPixelBufferError> for Detail {
    fn from(error: core_video_rs::cv_pixel_buffer::error::CVPixelBufferError) -> Self {
        Self {
            code: None,
            message: error.to_string(),
        }
    }
}
