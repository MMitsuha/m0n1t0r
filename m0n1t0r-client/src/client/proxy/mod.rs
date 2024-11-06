use m0n1t0r_common::proxy as mcproxy;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

impl mcproxy::Agent for AgentObj {}
