use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, System as SystemInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    uptime: u64,
    boot_time: u64,
    name: Option<String>,
    kernel_version: Option<String>,
    long_os_version: Option<String>,
    distribution_id: String,
    host_name: Option<String>,
    cpu_arch: String,
    cpu: Cpu,
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            uptime: SystemInfo::uptime(),
            boot_time: SystemInfo::boot_time(),
            name: SystemInfo::name(),
            kernel_version: SystemInfo::kernel_version(),
            long_os_version: SystemInfo::long_os_version(),
            distribution_id: SystemInfo::distribution_id(),
            host_name: SystemInfo::host_name(),
            cpu_arch: SystemInfo::cpu_arch(),
            cpu: Cpu::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cpu {
    count: HashMap<String, u32>,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        let info = SystemInfo::new_with_specifics(
            sysinfo::RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let cpus = info.cpus();
        cpus.iter()
            .map(|cpu| cpu.brand())
            .for_each(|brand| match cpu.count.get_mut(brand) {
                Some(count) => *count += 1,
                None => {
                    cpu.count.insert(brand.to_string(), 1);
                }
            });

        cpu
    }
}
