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
}

#[macro_export]
macro_rules! default_agents {
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
macro_rules! create_agent_instance {
    ($name:ident) => {{
        let server = Arc::new(RwLock::new($name::AgentObj::new()));
        let (server_server, server_client) =
            m0n1t0r_common::$name::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            if let Err(e) = server_server.serve(true).await {
                warn!("{}: serve error: {}", stringify!($name), e);
            }
        });
        Ok(server_client)
    }};
}
