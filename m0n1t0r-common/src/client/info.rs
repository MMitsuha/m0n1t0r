use serde::{Deserialize, Serialize};
use sysinfo::System as SysSystem;

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    uptime: u64,
    boot_time: u64,
    name: Option<String>,
    kernel_version: Option<String>,
    long_os_version: Option<String>,
    distribution_id: String,
    host_name: Option<String>,
    cpu_arch: Option<String>,
}

impl System {
    pub fn new() -> Self {
        Self {
            uptime: SysSystem::uptime(),
            boot_time: SysSystem::boot_time(),
            name: SysSystem::name(),
            kernel_version: SysSystem::kernel_version(),
            long_os_version: SysSystem::long_os_version(),
            distribution_id: SysSystem::distribution_id(),
            host_name: SysSystem::host_name(),
            cpu_arch: SysSystem::cpu_arch(),
        }
    }
}
