use crate::process::Process;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub async fn list() -> Vec<Process> {
    let process = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    let processes = process
        .processes()
        .into_iter()
        .map(|(pid, process)| Process::from_process(pid, process))
        .collect();

    processes
}
