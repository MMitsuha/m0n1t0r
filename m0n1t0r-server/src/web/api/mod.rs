mod client;
mod index;

use crate::{web::util, ServerMap};
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
        let (path_config, query_config) = util::extractor_config();

        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .app_data(Data::new(server_map.clone()))
            .app_data(path_config)
            .app_data(query_config)
            .service(index::get)
            .service(
                web::scope("/client")
                    .service(client::get)
                    .service(client::notify::get)
                    .service(
                        web::scope("/proxy")
                            .service(client::proxy::get)
                            .service(client::proxy::delete),
                    )
                    .service(
                        web::scope("/{addr}")
                            .service(client::client::get)
                            .service(client::client::environment::get)
                            .service(client::client::terminate::post)
                            .service(
                                web::scope("/update")
                                    .service(client::update::by_url::post)
                                    .service(client::update::by_file::post),
                            )
                            .service(
                                web::scope("/fs")
                                    .service(client::fs::metadata::get)
                                    .service(client::fs::get)
                                    .service(client::fs::delete)
                                    .service(client::fs::put),
                            )
                            .service(
                                web::scope("/process")
                                    .service(client::process::interactive::post)
                                    .service(client::process::execute::detached::post)
                                    .service(client::process::execute::post)
                                    .service(client::process::get)
                                    .service(client::process::delete),
                            )
                            .service(
                                web::scope("/proxy")
                                    .service(client::proxy::socks5::noauth::post)
                                    .service(client::proxy::socks5::pass::post),
                            )
                            .service(web::scope("/info").service(client::info::system::get))
                            .service(
                                web::scope("/network").service(client::network::download::post),
                            )
                            .service(
                                web::scope("/qq")
                                    .service(client::qq::get)
                                    .service(client::qq::url::get)
                                    .service(client::qq::friend::get),
                            ),
                    ),
            )
    })
    .bind(config.addr)?
    .run()
    .await?;
    Ok(())
}
