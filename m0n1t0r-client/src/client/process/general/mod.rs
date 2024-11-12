use m0n1t0r_common::process as mcprocess;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

impl mcprocess::Agent for AgentObj {}
