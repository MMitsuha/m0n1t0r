use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, Key, SlotMap};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub mod proxy {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Socks5 {
        pub from: SocketAddr,
        pub addr: SocketAddr,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Forward {
        pub from: SocketAddr,
        pub to: SocketAddr,
        pub addr: SocketAddr,
    }

    #[derive(Serialize, Deserialize)]
    pub enum Type {
        Socks5(Socks5),
        Forward(Forward),
    }

    #[derive(Serialize, Deserialize)]
    pub struct Proxy {
        pub key: u64,
        pub r#type: Type,
        #[serde(skip)]
        pub canceller: CancellationToken,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProxyMap(SlotMap<DefaultKey, Proxy>);

    impl Proxy {
        pub fn new(r#type: Type, canceller: CancellationToken) -> Self {
            Self {
                key: 0,
                r#type,
                canceller,
            }
        }

        pub fn set_key(&mut self, key: DefaultKey) {
            self.key = key.data().as_ffi();
        }
    }

    impl ProxyMap {
        pub fn new() -> Self {
            Self(SlotMap::new())
        }

        pub fn get(&self, key: DefaultKey) -> Option<&Proxy> {
            self.0.get(key)
        }

        pub fn insert(&mut self, proxy: Proxy) -> DefaultKey {
            let key = self.0.insert(proxy);
            self.0[key].set_key(key);
            key
        }

        pub fn remove(&mut self, key: DefaultKey) -> Option<Proxy> {
            self.0.remove(key)
        }

        pub fn as_vec(&self) -> Vec<&Proxy> {
            self.0.iter().map(|(_, proxy)| proxy).collect()
        }
    }

    impl From<(SocketAddr, &SocketAddr)> for Socks5 {
        fn from(tuple: (SocketAddr, &SocketAddr)) -> Self {
            Self {
                from: tuple.0,
                addr: *tuple.1,
            }
        }
    }

    impl From<(SocketAddr, SocketAddr, &SocketAddr)> for Forward {
        fn from(tuple: (SocketAddr, SocketAddr, &SocketAddr)) -> Self {
            Self {
                from: tuple.0,
                to: tuple.1,
                addr: *tuple.2,
            }
        }
    }

    lazy_static! {
        pub static ref PROXY_MAP: Arc<RwLock<ProxyMap>> = Arc::new(RwLock::new(ProxyMap::new()));
    }
}
