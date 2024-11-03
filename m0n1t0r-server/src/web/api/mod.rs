mod client;
mod index;

use crate::{Config as CrateConfig, ServerMap};
use actix_web::{middleware, web::Data, App, HttpServer};
use anyhow::Result;
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

pub async fn run(config: &Config) -> Result<()> {
    let server_map = config.server_map.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(server_map.clone()))
            .service(client::get)
            .service(client::info::get)
            .service(index::get)
    })
    .bind(config.addr)?
    .run()
    .await?;
    Ok(())
}
