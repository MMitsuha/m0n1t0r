mod client;
mod index;

use crate::ServerMap;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use middleware::Logger;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: SocketAddr,
    server_map: Arc<RwLock<ServerMap>>,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
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
            .wrap(Logger::default())
            .app_data(Data::new(server_map.clone()))
            .service(index::get)
            .service(
                web::scope("/client").service(client::get).service(
                    web::scope("/{addr}").service(client::client::get).service(
                        web::scope("/fs")
                            .service(client::fs::get)
                            .service(client::fs::delete)
                            .service(client::fs::put)
                            .service(client::fs::head),
                    ),
                ),
            )
    })
    .bind(config.addr)?
    .run()
    .await?;
    Ok(())
}
