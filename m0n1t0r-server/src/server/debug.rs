use crate::ServerObj;
use actix_web::web::Buf;
use anyhow::{Result, anyhow};
use log::info;
use m0n1t0r_common::{
    charset::Agent as _,
    client::{Client, TargetPlatform},
    fs::Agent as _,
    process::Agent as _,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run(server: Arc<RwLock<ServerObj>>) -> Result<()> {
    let lock_obj = server.read().await;
    let client = lock_obj.client()?;
    let file_agent = client.fs_agent().await?;
    let process_agent = client.process_agent().await?;
    let charset_agent = client.charset_agent().await?;
    let platform = client.target_platform().await?;
    let shell = client.shell().await?;

    info!("target platform: {:?}", platform);
    client.ping().await?;
    info!("version: {}", client.version().await?);
    info!(
        "pwd at: {}",
        file_agent.current_directory().await?.to_string_lossy()
    );
    info!("files at \"/\": {:?}", file_agent.list("/".into()).await?);
    info!("target shell: {:?}", shell);

    if platform == TargetPlatform::Linux && platform == TargetPlatform::MacOS {
        let (stdin_tx, stdout_rx, _) = process_agent.interactive("sh".to_string()).await?;
        let mut stdin_tx = stdin_tx.into_inner().await?;
        let mut stdout_rx = stdout_rx.into_inner().await?;
        stdin_tx.send("echo hello\n".into()).await?;
        assert_eq!(
            "hello\n",
            String::from_utf8_lossy(
                stdout_rx
                    .recv()
                    .await?
                    .ok_or(anyhow!("channel closed"))?
                    .chunk()
            )
        );
    }

    if platform == TargetPlatform::Windows {
        let charset = charset_agent.acp().await?;
        info!("current acp: {}", charset);
        // Make sure the system's acp is utf8
        if charset == 936 {
            let chinese_love = charset_agent.acp_to_utf8(vec![0xb0, 0xae]).await?;
            assert_eq!(chinese_love.as_bytes(), vec![0xe7, 0x88, 0xb1]);
        }
    }

    // Not testing autorun due to environment damage

    Ok(())
}
