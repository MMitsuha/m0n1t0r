use crate::ServerObj;
use actix_web::web::Buf;
use anyhow::{Error, Result, anyhow};
use hbb_common::{message_proto::VideoFrame, protobuf::Message};
use log::info;
use m0n1t0r_common::{
    charset::Agent as _,
    client::{Client, TargetPlatform},
    fs::Agent as _,
    process::Agent as _,
    rd::Agent,
};
use scrap::{
    CodecFormat, GoogleImage, Image, ImageFormat, ImageRgb, ImageTexture, STRIDE_ALIGN,
    codec::Decoder,
};
use std::{
    io::Write,
    process::{Command, Stdio},
    sync::Arc,
};
use tokio::{runtime::Handle, sync::RwLock, task};

pub async fn run(server: Arc<RwLock<ServerObj>>) -> Result<()> {
    let lock_obj = server.read().await;
    let client = lock_obj.client()?;
    let file_agent = client.fs_agent().await?;
    let process_agent = client.process_agent().await?;
    let charset_agent = client.charset_agent().await?;
    let rd_agent = client.rd_agent().await?;
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

    let displays = rd_agent.displays().await?;
    for display in &displays {
        info!("found display: {}", display);
    }
    let selected_display = displays
        .into_iter()
        .next()
        .ok_or(anyhow!("no display found"))?;
    let mut rx = rd_agent.view(selected_display.clone(), 0.5).await?;
    task::spawn_blocking(move || {
        let mut child = Command::new("ffplay")
            .args([
                "-f",
                "rawvideo",
                "-pixel_format",
                "bgr0",
                "-video_size",
                &format!("{}x{}", selected_display.width, selected_display.height),
                "-framerate",
                "60",
                "-",
            ])
            .stdin(Stdio::piped())
            .spawn()?;
        let mut out = child.stdin.take().unwrap();

        let mut decoder = Decoder::new(CodecFormat::VP9, None);
        let mut pixelbuffer = true;
        let mut chroma = None;
        let mut rgb = ImageRgb::new(ImageFormat::ARGB, STRIDE_ALIGN);
        let mut texture = ImageTexture::default();

        while let Ok(data) = Handle::current().block_on(rx.recv()) {
            let vf =
                VideoFrame::parse_from_bytes(data.ok_or(anyhow!("channel closed"))?.as_slice())?;
            if let Some(frame) = vf.union {
                decoder.handle_video_frame(
                    &frame,
                    &mut rgb,
                    &mut texture,
                    &mut pixelbuffer,
                    &mut chroma,
                )?;

                if pixelbuffer {
                    for row in rgb.raw.chunks(<Image as GoogleImage>::get_bytes_per_row(
                        rgb.w, rgb.fmt, rgb.align,
                    )) {
                        out.write_all(row)?;
                    }
                }
            }
        }
        Ok::<(), Error>(())
    });

    // Not testing autorun due to environment damage

    Ok(())
}
