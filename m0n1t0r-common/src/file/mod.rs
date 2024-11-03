use crate::Result as AppResult;
use anyhow::{Error, Result};
use remoc::rtc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{self, DirEntry};

#[rtc::remote]
pub trait Agent: Sync {
    async fn list(&self, path: PathBuf) -> AppResult<Vec<File>> {
        let mut entries = fs::read_dir(path).await.map_err(Error::from)?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await.map_err(Error::from)? {
            files.push(File::from_dir_entry(&entry).await?);
        }
        Ok(files)
    }

    async fn read(&self, path: PathBuf) -> AppResult<Vec<u8>> {
        Ok(fs::read(path).await.map_err(Error::from)?)
    }

    async fn current_directory(&self) -> AppResult<PathBuf> {
        Ok(fs::canonicalize(".").await.map_err(Error::from)?)
    }

    async fn write(&self, path: PathBuf, data: Vec<u8>) -> AppResult<()> {
        Ok(fs::write(path, data).await.map_err(Error::from)?)
    }

    async fn create_directory(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::create_dir(path).await.map_err(Error::from)?)
    }

    async fn remove_file(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::remove_file(path).await.map_err(Error::from)?)
    }

    async fn remove_directory(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::remove_dir_all(path).await.map_err(Error::from)?)
    }

    async fn rename(&self, from: PathBuf, to: PathBuf) -> AppResult<()> {
        Ok(fs::rename(from, to).await.map_err(Error::from)?)
    }

    async fn copy(&self, from: PathBuf, to: PathBuf) -> AppResult<u64> {
        Ok(fs::copy(from, to).await.map_err(Error::from)?)
    }

    async fn symlink(&self, from: PathBuf, to: PathBuf) -> AppResult<()> {
        Ok(fs::symlink(from, to).await.map_err(Error::from)?)
    }

    async fn hardlink(&self, from: PathBuf, to: PathBuf) -> AppResult<()> {
        Ok(fs::hard_link(from, to).await.map_err(Error::from)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub is_symlink: bool,
}

impl File {
    async fn from_dir_entry(entry: &DirEntry) -> Result<Self> {
        let r#type = entry.file_type().await?;
        let metadata = entry.metadata().await?;

        Ok(Self {
            name: entry.file_name().to_string_lossy().into(),
            size: metadata.len(),
            is_dir: r#type.is_dir(),
            is_symlink: r#type.is_symlink(),
        })
    }
}
