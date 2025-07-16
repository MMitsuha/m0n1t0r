use crate::web::{Error, Result as WebResult};
use discriminant_rs::Discriminant;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Response {
    code: i16,
    body: Value,
}

impl Default for Response {
    fn default() -> Self {
        Self::error(Error::Unknown).unwrap()
    }
}

impl Response {
    fn new(code: i16, body: impl Serialize) -> WebResult<Self> {
        Ok(Self {
            code,
            body: serde_json::to_value(body)?,
        })
    }

    pub fn success(body: impl Serialize) -> WebResult<Self> {
        Self::new(Error::Okay.discriminant(), body)
    }

    pub fn error(error: Error) -> WebResult<Self> {
        Self::new(error.discriminant(), error.to_string())
    }
}
