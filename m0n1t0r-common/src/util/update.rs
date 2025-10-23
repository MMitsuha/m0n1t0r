use crate::{Result as AppResult, process};
use std::{env, path::PathBuf};
use tokio::fs as tfs;

pub async fn update(temp: PathBuf) -> AppResult<()> {
    self_replace::self_replace(&temp)?;
    tfs::remove_file(&temp).await?;
    process::execute::execute_detached(
        env::current_exe()?.to_string_lossy().to_string(),
        Vec::new(),
    )?;

    Ok(())
}
