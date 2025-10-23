use crate::{
    ServerMap,
    web::{Error, Response, Result as WebResult},
};
use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use actix_web::{
    Responder, post,
    web::{Data, Form, Json, Path},
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
        let client = lock_obj.client()?;

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
        let client = lock_obj.client()?;

        Ok(Json(Response::success(
            client
                .update_by_file(
                    form.file.data.to_vec(),
                    form.temp
                        .unwrap_or(Text(TEMP_FILE.into()))
                        .into_inner()
                        .into(),
                )
                .await?,
        )?))
    }
}
