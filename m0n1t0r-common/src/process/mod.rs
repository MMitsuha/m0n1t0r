pub mod execute;
mod interactive;
mod list;

use crate::Result as AppResult;
use remoc::{
    rch::bin::{Receiver, Sender},
    rtc,
};
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

#[rtc::remote]
pub trait Agent: Sync {
    async fn execute(&self, command: String, args: Vec<String>) -> AppResult<execute::Output> {
        execute::execute(command, args).await
    }

    async fn interactive(&self, command: String) -> AppResult<(Sender, Receiver, Receiver)> {
        interactive::interactive(command).await
    }

    async fn list(&self) -> AppResult<Vec<Process>> {
        Ok(list::list().await)
    }

    async fn kill_by_pid(&self, pid: u32) -> AppResult<Vec<Process>> {
        Ok(kill_by(|p, _| p.as_u32() == pid)?)
    }

    async fn kill_by_name(&self, name: String) -> AppResult<Vec<Process>> {
        Ok(kill_by(|_, process| {
            process.name().to_string_lossy() == name
        })?)
    }
}

fn kill_by<F>(function: F) -> AppResult<Vec<Process>>
where
    F: Fn(&&Pid, &&SysProcess) -> bool,
{
    let process = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    let processes = process
        .processes()
        .into_iter()
        .filter(|(pid, process)| function(pid, process))
        .map(|(pid, process)| {
            process.kill();
            Process::from_process(pid, process)
        })
        .collect();

    Ok(processes)
}
