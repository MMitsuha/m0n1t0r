use crate::web::Error;
use actix_web::web::{PathConfig, QueryConfig};
use actix_ws::{CloseCode, Session};
use log::warn;
use std::future::Future;

pub fn extractor_config() -> (PathConfig, QueryConfig) {
    (
        PathConfig::default().error_handler(|error, _| Error::from(error).into()),
        QueryConfig::default().error_handler(|error, _| Error::from(error).into()),
    )
}

pub async fn handle_websocket<F>(session: Session, future: F)
where
    F: Future<Output = anyhow::Result<()>> + 'static,
{
    let _ = match future.await {
        Ok(_) => session.close(None).await,
        Err(e) => {
            warn!("websocket error: {}", e);
            session
                .close(Some((CloseCode::Abnormal, e.to_string()).into()))
                .await
        }
    };
}
