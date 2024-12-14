use super::*;

impl From<windows_capture::graphics_capture_api::Error> for Detail {
    fn from(value: windows_capture::graphics_capture_api::Error) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}

impl From<windows_capture::monitor::Error> for Detail {
    fn from(value: windows_capture::monitor::Error) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}

impl From<windows_capture::capture::GraphicsCaptureApiError<Error>> for Detail {
    fn from(value: windows_capture::capture::GraphicsCaptureApiError<Error>) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}

impl From<windows_capture::capture::CaptureControlError<Error>> for Detail {
    fn from(value: windows_capture::capture::CaptureControlError<Error>) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}

impl From<windows_capture::frame::Error> for Detail {
    fn from(value: windows_capture::frame::Error) -> Self {
        Self {
            code: None,
            message: value.to_string(),
        }
    }
}
