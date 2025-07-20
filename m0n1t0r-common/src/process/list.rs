use crate::process::Process;
use rayon::prelude::{IntoParallelIterator, ParallelIterator as _};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub async fn list() -> Vec<Process> {
    let process = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    

    process
        .processes()
        .into_par_iter()
        .map(|(pid, process)| Process::from_process(pid, process))
        .collect()
}
