pub mod api;
mod error;

use actix_web::web::{PathConfig, QueryConfig};
pub use error::*;

use anyhow::anyhow;
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
            code: Error::from(anyhow!("unable to display error")).discriminant(),
            body: Value::Null,
        }
    }
}

impl Response {
    fn new(code: i16, body: impl Serialize) -> WebResult<Self> {
        Ok(Self {
            code,
            body: serde_json::to_value(body)?,
        })
    }

    fn success(body: impl Serialize) -> WebResult<Self> {
        Self::new(Error::Okay.discriminant(), body)
    }

    fn error(error: Error) -> WebResult<Self> {
        Self::new(error.discriminant(), error.to_string())
    }
}

fn extractor_config() -> (PathConfig, QueryConfig) {
    (
        PathConfig::default().error_handler(|error, _| Error::from(error).into()),
        QueryConfig::default().error_handler(|error, _| Error::from(error).into()),
    )
}
