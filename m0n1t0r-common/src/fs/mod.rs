use crate::Result as AppResult;
use remoc::rtc;
use serde::{Deserialize, Serialize};
use std::{env, fs::Metadata, path::PathBuf};
use tokio::{
    fs::{self, DirEntry},
    io::AsyncWriteExt,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub is_symlink: bool,
}

impl File {
    async fn from_dir_entry(entry: &DirEntry) -> AppResult<Self> {
        let metadata = entry.metadata().await?;

        Ok(Self::from_metadata(&metadata, &entry.path()))
    }

    fn from_metadata(metadata: &Metadata, path: &PathBuf) -> Self {
        Self {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.clone(),
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
        }
    }
}

#[rtc::remote]
pub trait Agent: Sync {
    async fn list(&self, path: PathBuf) -> AppResult<Vec<File>> {
        let mut entries = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            files.push(File::from_dir_entry(&entry).await?);
        }
        Ok(files)
    }

    async fn list_recursive(&self, path: PathBuf) -> AppResult<Vec<File>> {
        let mut entries = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            files.push(File::from_dir_entry(&entry).await?);

            if entry.metadata().await?.is_dir() {
                files.extend(self.list_recursive(entry.path()).await?);
            }
        }
        Ok(files)
    }

    async fn read(&self, path: PathBuf) -> AppResult<Vec<u8>> {
        Ok(fs::read(path).await?)
    }

    async fn current_directory(&self) -> AppResult<PathBuf> {
        Ok(env::current_dir()?)
    }

    async fn current_exe(&self) -> AppResult<PathBuf> {
        Ok(env::current_exe()?)
    }

    async fn write(&self, path: PathBuf, data: Vec<u8>) -> AppResult<()> {
        Ok(fs::write(path, data).await?)
    }

    async fn append(&self, path: PathBuf, data: Vec<u8>) -> AppResult<()> {
        fs::OpenOptions::new()
            .append(true)
            .open(path)
            .await?
            .write(&data)
            .await?;
        Ok(())
    }

    async fn create_directory(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::create_dir(path).await?)
    }

    async fn create_directory_all(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::create_dir_all(path).await?)
    }

    async fn remove_file(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::remove_file(path).await?)
    }

    async fn remove_directory(&self, path: PathBuf) -> AppResult<()> {
        Ok(fs::remove_dir_all(path).await?)
    }

    async fn rename(&self, from: PathBuf, to: PathBuf) -> AppResult<()> {
        Ok(fs::rename(from, to).await?)
    }

    async fn copy(&self, from: PathBuf, to: PathBuf) -> AppResult<u64> {
        Ok(fs::copy(from, to).await?)
    }

    async fn hardlink(&self, from: PathBuf, to: PathBuf) -> AppResult<()> {
        Ok(fs::hard_link(from, to).await?)
    }

    async fn file(&self, path: PathBuf) -> AppResult<File> {
        let metadata = fs::metadata(&path).await?;
        Ok(File::from_metadata(&metadata, &path))
    }

    async fn symlink_file(&self, path: PathBuf) -> AppResult<File> {
        let metadata = fs::symlink_metadata(&path).await?;
        Ok(File::from_metadata(&metadata, &path))
    }

    async fn is_dir(&self, path: PathBuf) -> AppResult<bool> {
        Ok(fs::metadata(path).await?.is_dir())
    }

    async fn is_symlink(&self, path: PathBuf) -> AppResult<bool> {
        Ok(fs::metadata(path).await?.is_symlink())
    }
}
