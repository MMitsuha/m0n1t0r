#[macro_export]
macro_rules! declare_agent {
    ($name:ident) => {
        mod $name {
            pub struct AgentObj {}
            impl AgentObj {
                pub fn new() -> Self {
                    Self {}
                }
            }
            impl m0n1t0r_common::$name::Agent for AgentObj {}
        }
    };
}

#[macro_export]
macro_rules! declare_agents {
    ($($name:ident), *) => {
        $(declare_agent!($name);)*
    };
}

#[macro_export]
macro_rules! declare_agents_with_platform {
    (windows, $($name:ident), *) => {
        $(
            #[cfg(all(
                feature = "general",
                not(feature = "windows"),
                not(feature = "linux"),
                not(feature = "macos"),
            ))]
            declare_agent!($name);

            #[cfg(feature = "windows")]
            use windows::$name;
        )*
    };
}

#[macro_export]
macro_rules! impl_agent {
    ($name:ident) => {{
        let server = Arc::new(RwLock::new($name::AgentObj::new()));
        let (server_server, server_client) =
            m0n1t0r_common::$name::AgentServerSharedMut::<_>::new(server.clone(), 1);

        tokio::spawn(async move {
            server_server.serve(true).await;
        });
        Ok(server_client)
    }};
}
