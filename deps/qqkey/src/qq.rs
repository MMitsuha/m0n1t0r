use crate::{Account, AccountInfoList, AccountList, Error, Result as QQResult, LOGIN_REFERER};
use rand::Rng;
use regex::Regex;
use reqwest::{header::REFERER, Client, Proxy};
use reqwest_cookie_store::{CookieStore, CookieStoreRwLock};
use std::sync::Arc;

#[derive(Debug)]
pub struct QQ {
    pub(crate) client: Client,
    pub(crate) cookie_store: Arc<CookieStoreRwLock>,
    pub(crate) local_token: String,
}

impl QQ {
    pub async fn new() -> QQResult<Self> {
        let cookie_store = Arc::new(CookieStoreRwLock::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()?;
        let local_token = Self::get_local_token(&client, cookie_store.clone()).await?;

        Ok(Self {
            client,
            cookie_store,
            local_token,
        })
    }

    pub async fn new_with_proxy(proxy: Proxy) -> QQResult<Self> {
        let cookie_store = Arc::new(CookieStoreRwLock::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .proxy(proxy)
            .build()?;
        let local_token = Self::get_local_token(&client, cookie_store.clone()).await?;

        Ok(Self {
            client,
            cookie_store,
            local_token,
        })
    }

    async fn get_local_token(
        client: &Client,
        cookie_store: Arc<CookieStoreRwLock>,
    ) -> QQResult<String> {
        let url= "https://xui.ptlogin2.qq.com/cgi-bin/xlogin?appid=1006102&s_url=http://id.qq.com/index.html";

        if client.get(url).send().await?.status().is_success() == false {
            return Err(Error::QQError);
        }

        Ok(cookie_store
            .read()?
            .get("ptlogin2.qq.com", "/", "pt_local_token")
            .ok_or(Error::CookieNotFound("pt_local_token".into()))?
            .value()
            .to_string())
    }

    pub async fn get_logged_qq_info(&self) -> QQResult<AccountInfoList> {
        let url = format!("https://localhost.ptlogin2.qq.com:4301/pt_get_uins?callback=ptui_getuins_CB&r={}&pt_local_tk={}", rand::rng().random_range(0.0..1.0), self.local_token);
        let response = self
            .client
            .get(url)
            .header(REFERER, LOGIN_REFERER)
            .send()
            .await?
            .text()
            .await?;
        let response = &Regex::new(r"(?<json>\[.*\])")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("qq info".into()))?["json"];

        Ok(serde_json::from_str(response)?)
    }

    pub async fn get_logged_qq(&self) -> QQResult<AccountList> {
        let info = self.get_logged_qq_info().await?;
        let mut ret = AccountList::new();

        for i in info {
            ret.push(Account::from(self, i).await?);
        }

        Ok(ret)
    }
}
