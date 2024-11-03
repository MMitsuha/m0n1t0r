use crate::ServerObj;
use anyhow::Result;
use m0n1t0r_common::{client::Client, file::Agent as _, process::Agent as _, util};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn debug(server: Arc<RwLock<ServerObj>>) -> Result<()> {
    let lock = server.read().await;
    let client = lock.get_client()?;
    let file_agent = client.get_file_agent().await?;
    let process_agent = client.get_process_agent().await?;

    println!(
        "ping: {}ms",
        client.ping(util::time::ntp().await?).await?.as_millis()
    );
    println!(
        "pwd: {}",
        String::from_utf8_lossy(
            &process_agent
                .execute("pwd".into(), Vec::new())
                .await?
                .stdout
        )
    );
    println!(
        "ls -l: {}",
        String::from_utf8_lossy(
            &process_agent
                .execute("ls".into(), vec!["-l".into()])
                .await?
                .stdout
        )
    );
    println!(
        "pwd at: {}",
        file_agent.current_directory().await?.to_string_lossy()
    );
    println!(
        "files at \"/\": {:?}",
        file_agent.list_files("/".into()).await?
    );
    println!(
        "Cargo.toml: \"{}\"",
        String::from_utf8_lossy(&file_agent.read_file("Cargo.toml".into()).await?)
    );

    Ok(())
}
