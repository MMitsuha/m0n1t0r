use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sysinfo::{Pid, Process as SysProcess, ProcessRefreshKind, RefreshKind, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub name: String,
    cmd: Vec<String>,
    exe: Option<PathBuf>,
    pid: usize,
}

impl Process {
    fn from_process(pid: &Pid, process: &SysProcess) -> Self {
        Self {
            name: process.name().to_string_lossy().to_string(),
            cmd: process
                .cmd()
                .iter()
                .map(|c| c.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            exe: process.exe().map(|p| p.to_path_buf()),
            pid: pid.as_u32() as usize,
        }
    }
}

pub async fn list() -> Vec<Process> {
    let process = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    let mut processes = Vec::new();

    for (pid, process) in process.processes() {
        processes.push(Process::from_process(pid, process));
    }

    processes
}
