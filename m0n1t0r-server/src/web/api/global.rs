use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub mod proxy {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Copy)]
    pub enum Type {
        Socks5,
    }

    lazy_static! {
        pub static ref PROXY_MAP: Arc<RwLock<HashMap<SocketAddr, (CancellationToken, Type)>>> =
            Arc::new(RwLock::new(HashMap::new()));
    }
}
