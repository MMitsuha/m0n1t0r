use m0n1t0r_common::fs as mcfile;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

impl mcfile::Agent for AgentObj {}
