#[macro_export]
macro_rules! declare_agents {
    ($module:ident, [$($name:ident),+], [$($platform:literal),+]) => {
        #[cfg(any(
            $(feature = $platform),*
        ))]
        $(
            use $module::$name;
        )*
    };
    ([$($name:ident),+]) => {
        $(
            #[allow(dead_code)]
            pub mod $name {
                pub struct AgentObj {}
                impl AgentObj {
                    pub fn new() -> Self {
                        Self {}
                    }
                }
                impl m0n1t0r_common::$name::Agent for AgentObj {}
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_agent {
    ($name:ident, $s:ident) => {{
        let server = Arc::new(RwLock::new($name::AgentObj::new()));
        let (server_server, server_client) =
            m0n1t0r_common::$name::AgentServerSharedMut::<_>::new(server.clone(), 1);
        let addr = $s.addr;

        tokio::spawn(async move {
            if let Err(e) = server_server.serve(true).await {
                warn!("{}({}): serve error: {}", addr, stringify!($name), e);
            }
        });
        Ok(server_client)
    }};
}
