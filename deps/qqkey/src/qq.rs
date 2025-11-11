use crate::{Error, Result as QQResult, LOGIN_REFERER, QQ_REFERER};
use rand::Rng;
use regex::Regex;
use reqwest::{header::REFERER, Client, IntoUrl, Method, RequestBuilder};
use reqwest_cookie_store::{CookieStore, CookieStoreRwLock};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct QQ {
    client: Client,
    cookie_store: Arc<CookieStoreRwLock>,
    local_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub uin: i64,
    pub face_index: i64,
    pub gender: i64,
    pub nickname: Option<String>,
    pub client_type: i64,
    pub uin_flag: i64,
    pub account: i64,
}

impl QQ {
    pub async fn new() -> QQResult<Self> {
        let cookie_store = Arc::new(CookieStoreRwLock::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()?;

        Self {
            client,
            cookie_store,
            local_token: None,
        }
        .initialize()
        .await
    }

    pub(crate) fn read_cookie(&self, domain: &str, path: &str, name: &str) -> QQResult<String> {
        Ok(self
            .cookie_store
            .read()?
            .get(domain, path, name)
            .ok_or(Error::CookieNotFound(name.into()))?
            .value()
            .to_string())
    }

    async fn initialize(mut self) -> QQResult<Self> {
        let url= "https://xui.ptlogin2.qq.com/cgi-bin/xlogin?appid=1006102&s_url=http://id.qq.com/index.html";

        if !self
            .login_request(Method::GET, url)
            .send()
            .await?
            .status()
            .is_success()
        {
            return Err(Error::RequestQQError);
        }
        self.local_token = Some(self.read_cookie("ptlogin2.qq.com", "/", "pt_local_token")?);

        Ok(self)
    }

    pub(crate) fn local_token(&self) -> String {
        self.local_token.clone().unwrap()
    }

    pub(crate) fn login_request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client
            .request(method, url)
            .header(REFERER, LOGIN_REFERER)
    }

    pub(crate) fn qq_request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client.request(method, url).header(REFERER, QQ_REFERER)
    }

    pub async fn logged_qq(&self) -> QQResult<Vec<Info>> {
        let url = format!("https://localhost.ptlogin2.qq.com:4301/pt_get_uins?callback=ptui_getuins_CB&r={}&pt_local_tk={}", rand::rng().random_range(0.0..1.0), self.local_token());
        let response = self
            .login_request(Method::GET, url)
            .send()
            .await?
            .text()
            .await?;
        let response = &Regex::new(r"(?<json>\[.*\])")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("qq info".into()))?["json"];

        Ok(serde_json::from_str(response)?)
    }
}
