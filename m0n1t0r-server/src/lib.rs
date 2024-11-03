mod conn;
mod server;
mod web;

pub use conn::ServerMap;
pub use server::ServerObj;

use anyhow::Result;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{select, sync::RwLock};
use web::api;

pub struct Config {
    conn_addr: SocketAddr,
    api_addr: SocketAddr,
    server_map: Arc<RwLock<ServerMap>>,
}

impl Config {
    pub fn new(conn_addr: &SocketAddr, api_addr: &SocketAddr) -> Self {
        Self {
            conn_addr: conn_addr.clone(),
            api_addr: api_addr.clone(),
            server_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn run(config: &Config) -> Result<()> {
    let conn_config = config.into();
    let api_config = config.into();

    select! {
        conn = conn::run(&conn_config) => conn?,
        api = api::run(&api_config) => api?,
    };
    Ok(())
}
