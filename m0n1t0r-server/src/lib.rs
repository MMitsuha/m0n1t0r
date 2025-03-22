#[allow(unused_imports)]
#[macro_use]
extern crate m0n1t0r_macro;

mod conn;
mod db;
mod server;
mod web;

pub use conn::ServerMap;
pub use server::ServerObj;

use anyhow::Result;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use std::{net::SocketAddr, path::Path, sync::Arc};
use tokio::{select, sync::RwLock};
use web::api;

pub struct Config {
    conn_addr: SocketAddr,
    api_addr: SocketAddr,
    tls_config: rustls::ServerConfig,
}

impl Config {
    pub fn new(
        conn_addr: &SocketAddr,
        api_addr: &SocketAddr,
        key: &Path,
        cert: &Path,
    ) -> Result<Self> {
        let certs = CertificateDer::pem_file_iter(cert)?.collect::<Result<Vec<_>, _>>()?;
        let key = PrivateKeyDer::from_pem_file(key)?;
        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(Self {
            conn_addr: conn_addr.clone(),
            api_addr: api_addr.clone(),
            tls_config,
        })
    }
}

pub async fn run(config: &Config, server_map: Arc<RwLock<ServerMap>>) -> Result<()> {
    let conn_config = config.into();
    let api_config = config.into();

    select! {
        conn = conn::run(&conn_config, server_map.clone()) => conn?,
        api = api::run(&api_config, server_map.clone()) => api?,
    };
    Ok(())
}
