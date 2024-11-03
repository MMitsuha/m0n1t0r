mod client;
mod index;

use crate::{Config as CrateConfig, ServerMap};
use actix_web::{middleware, web::Data, App, HttpServer, Result as WebResult};
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: SocketAddr,
    server_map: Arc<RwLock<ServerMap>>,
}

impl From<&CrateConfig> for Config {
    fn from(config: &CrateConfig) -> Self {
        Self {
            addr: config.api_addr,
            server_map: config.server_map.clone(),
        }
    }
}

#[derive(Serialize)]
struct Response {
    code: i16,
    body: Value,
}

impl Response {
    fn new(code: i16, body: impl Serialize) -> WebResult<Self> {
        Ok(Self {
            code,
            body: serde_json::to_value(body)?,
        })
    }

    fn success(body: impl Serialize) -> WebResult<Self> {
        Self::new(0, body)
    }
}
pub async fn run(config: &Config) -> Result<()> {
    let server_map = config.server_map.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(server_map.clone()))
            .service(client::count::get)
            .service(index::get)
    })
    .bind(config.addr)?
    .run()
    .await?;
    Ok(())
}
