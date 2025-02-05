use crate::{
    web::{Error, Response, Result as WebResult},
    ServerMap,
};
use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use actix_web::{
    post,
    web::{Data, Form, Json, Path},
    Responder,
};
use m0n1t0r_common::client::Client as _;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

const TEMP_FILE: &str = "temp.bin";

pub mod by_url {
    use super::*;

    #[derive(Deserialize)]
    struct ByUrlForm {
        url: Url,
        temp: Option<PathBuf>,
    }

    #[post("/byurl")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        Form(form): Form<ByUrlForm>,
    ) -> WebResult<impl Responder> {
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(
            client
                .update_by_url(form.url, form.temp.unwrap_or(TEMP_FILE.into()))
                .await?,
        )?))
    }
}

pub mod by_file {
    use super::*;

    #[derive(MultipartForm)]
    struct ByFileForm {
        #[multipart(limit = "50MB")]
        file: Bytes,
        temp: Option<Text<String>>,
    }

    #[post("/byfile")]
    pub async fn post(
        data: Data<Arc<RwLock<ServerMap>>>,
        addr: Path<SocketAddr>,
        MultipartForm(form): MultipartForm<ByFileForm>,
    ) -> WebResult<impl Responder> {
        let lock_map = &data.read().await.map;
        let server = lock_map.get(&addr).ok_or(Error::NotFound)?;

        let lock_obj = server.read().await;
        let client = lock_obj.get_client()?;

        Ok(Json(Response::success(
            client
                .update_by_file(
                    form.file.data.to_vec(),
                    form.temp
                        .unwrap_or(Text {
                            0: TEMP_FILE.into(),
                        })
                        .into_inner()
                        .into(),
                )
                .await?,
        )?))
    }
}
