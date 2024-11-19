mod client;
mod index;

use crate::ServerMap;
use actix_web::{
    middleware::{self, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use middleware::Logger;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: SocketAddr,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
        Self {
            addr: config.api_addr,
        }
    }
}

pub async fn run(config: &Config, server_map: Arc<RwLock<ServerMap>>) -> Result<()> {
    HttpServer::new(move || {
        let (path_config, query_config) = super::extractor_config();

        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .app_data(Data::new(server_map.clone()))
            .app_data(path_config)
            .app_data(query_config)
            .service(index::get)
            .service(
                web::scope("/client").service(client::get).service(
                    web::scope("/{addr}")
                        .service(client::client::get)
                        .service(client::client::get_update)
                        .service(
                            web::scope("/fs")
                                .service(client::fs::get)
                                .service(client::fs::delete)
                                .service(client::fs::put)
                                .service(client::fs::head),
                        )
                        .service(
                            web::scope("/process")
                                .service(client::process::interactive::get)
                                .service(client::process::execute::get)
                                .service(client::process::get),
                        )
                        .service(
                            web::scope("/proxy")
                                .service(client::proxy::socks5::get_auth_none)
                                .service(client::proxy::socks5::get_auth_pass),
                        )
                        .service(web::scope("/network").service(client::network::download::get)),
                ),
            )
    })
    .bind(config.addr)?
    .run()
    .await?;
    Ok(())
}
