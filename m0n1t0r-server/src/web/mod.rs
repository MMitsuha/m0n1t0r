pub mod api;
mod error;

use error::{Error, Result as WebResult};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct Response {
    code: i16,
    body: Value,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            code: Error::Unknown as i16,
            body: Value::Null,
        }
    }
}

impl Response {
    fn new(code: i16, body: impl Serialize) -> WebResult<Self> {
        Ok(Self {
            code,
            body: serde_json::to_value(body).map_err(|_| Error::SerializeError)?,
        })
    }

    fn success(body: impl Serialize) -> WebResult<Self> {
        Self::new(Error::Okay as i16, body)
    }

    fn error(error: Error) -> WebResult<Self> {
        Self::new(error as i16, error)
    }
}
