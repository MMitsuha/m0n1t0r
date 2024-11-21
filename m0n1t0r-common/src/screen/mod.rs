use crate::Result as AppResult;
use remoc::{
    rch::bin::{self, Receiver},
    rtc,
};
use scap::capturer::{Capturer, Options};

#[rtc::remote]
pub trait Agent: Sync {
    async fn record(&self, options: Options) -> AppResult<Receiver> {
        let (tx, remote_rx) = bin::channel();

        tokio::spawn(async move {
            let mut recorder = Capturer::build(options)?;
            let mut tx = tx.into_inner().await?;

            recorder.start_capture();
            loop {
                let frame = recorder.get_next_frame()?;
                tx.send(rmp_serde::to_vec(&frame)?.into()).await?;
            }

            #[allow(unreachable_code)]
            Ok::<_, anyhow::Error>(())
        });

        Ok(remote_rx)
    }

    async fn is_supported(&self) -> AppResult<bool> {
        Ok(scap::is_supported())
    }

    async fn has_permission(&self) -> AppResult<bool> {
        Ok(scap::has_permission())
    }

    async fn request_permission(&self) -> AppResult<bool> {
        Ok(scap::request_permission())
    }

    async fn availability(&self) -> AppResult<bool> {
        Ok(scap::is_supported() && scap::has_permission())
    }
}
