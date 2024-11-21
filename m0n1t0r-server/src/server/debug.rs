use crate::ServerObj;
use actix_web::web::Buf;
use anyhow::{anyhow, Result};
use m0n1t0r_common::{client::Client, fs::Agent as _, process::Agent as _};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run(server: Arc<RwLock<ServerObj>>) -> Result<()> {
    let lock_obj = server.read().await;
    let client = lock_obj.get_client()?;
    let file_agent = client.get_fs_agent().await?;
    let process_agent = client.get_process_agent().await?;

    client.ping().await?;
    println!("version: {}", client.version().await?);
    println!(
        "pwd at: {}",
        file_agent.current_directory().await?.to_string_lossy()
    );
    println!("files at \"/\": {:?}", file_agent.list("/".into()).await?);
    println!(
        "Cargo.toml: \"{}\"",
        String::from_utf8_lossy(&file_agent.read("Cargo.toml".into()).await?)
    );

    let (stdin_tx, stdout_rx, _) = process_agent.interactive("sh".to_string()).await?;
    let mut stdin_tx = stdin_tx.into_inner().await?;
    let mut stdout_rx = stdout_rx.into_inner().await?;
    stdin_tx.send("echo hello\n".into()).await?;
    println!(
        "echo hello: {}",
        String::from_utf8_lossy(
            stdout_rx
                .recv()
                .await?
                .ok_or(anyhow!("channel closed"))?
                .chunk()
        )
    );

    Ok(())
}
