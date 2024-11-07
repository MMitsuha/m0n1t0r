use m0n1t0r_common::network as mcnetwork;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

impl mcnetwork::Agent for AgentObj {}
