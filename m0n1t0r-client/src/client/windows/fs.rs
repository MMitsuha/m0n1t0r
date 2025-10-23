use bit_iter::BitIter;
use m0n1t0r_common::Result as AppResult;
use m0n1t0r_common::fs::File;
use remoc::rtc;
use std::ops::Index;
use windows::Win32::Storage::FileSystem;

pub struct AgentObj {}

impl AgentObj {
    pub fn new() -> Self {
        Self {}
    }
}

#[rtc::async_trait]
impl m0n1t0r_common::fs::Agent for AgentObj {
    async fn drives(&self) -> AppResult<Vec<File>> {
        let drives = unsafe { FileSystem::GetLogicalDrives() };
        let letters = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        Ok(BitIter::from(drives)
            .map(|b| File::from_drive_letter(letters.index(b)))
            .collect())
    }
}
