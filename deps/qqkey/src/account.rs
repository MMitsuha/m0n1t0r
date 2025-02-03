use crate::{
    group::{Group, GroupList, ListResponse, Role},
    qzone::QZone,
    Error, Result as QQResult, LOGIN_REFERER, QQ, QQ_REFERER,
};
use rand::Rng;
use regex::Regex;
use reqwest::{header::REFERER, Client, Url};
use reqwest_cookie_store::CookieStoreRwLock;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};

pub type InfoList = Vec<Info>;
pub type AccountList = Vec<Account>;

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

pub struct Account {
    client: Client,
    cookie_store: Arc<CookieStoreRwLock>,
    info: Info,
    index: i32,
    key: String,
    skey_map: Arc<RwLock<HashMap<String, (String, String)>>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UrlList {
    pub mail_url: String,
    pub qzone_url: String,
    pub weiyun_url: String,
    pub qun_url: String,
}

impl UrlList {
    pub async fn new(account: &Account) -> QQResult<Self> {
        Ok(Self {
            mail_url: account.get_mail_url(),
            qzone_url: account.get_qzone_url(),
            weiyun_url: account.get_weiyun_url(),
            qun_url: account.get_qun_url().await?,
        })
    }
}

impl Account {
    pub(crate) async fn new(
        client: Client,
        cookie_store: Arc<CookieStoreRwLock>,
        info: Info,
        local_token: &String,
    ) -> QQResult<Self> {
        let (index, key) =
            Self::get_client_key(&client, cookie_store.clone(), &info, local_token).await?;

        Ok(Self {
            client,
            cookie_store,
            info,
            index,
            key,
            skey_map: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn from(qq: &QQ, info: Info) -> QQResult<Self> {
        Self::new(
            qq.client.clone(),
            qq.cookie_store.clone(),
            info,
            &qq.local_token,
        )
        .await
    }

    async fn get_client_key(
        client: &Client,
        cookie_store: Arc<CookieStoreRwLock>,
        info: &Info,
        local_token: &String,
    ) -> QQResult<(i32, String)> {
        let url = format!("https://localhost.ptlogin2.qq.com:4301/pt_get_st?clientuin={}&r={}&pt_local_tk={}&callback=ptui_getst_CB",info.uin, rand::rng().random_range(0.0..1.0),local_token);
        let response = client
            .get(url)
            .header(REFERER, LOGIN_REFERER)
            .send()
            .await?
            .text()
            .await?;
        let index = Regex::new(r"(?<index>\d*)};")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("client key".into()))?["index"]
            .parse()?;
        let client_key = cookie_store
            .read()?
            .get("ptlogin2.qq.com", "/", "clientkey")
            .ok_or(Error::CookieNotFound("clientkey".into()))?
            .value()
            .to_string();

        Ok((index, client_key))
    }

    fn get_hash(skey: &String) -> QQResult<String> {
        let mut t = 5381_i32;
        let o = skey.len();

        for n in 0..o {
            (t, _) = t.overflowing_add(
                (t << 5) + skey.chars().nth(n).ok_or(Error::InvalidCharacter)? as i32,
            );
        }

        Ok((t & 0x7fffffff).to_string())
    }

    fn login_url(&self, url: &str) -> String {
        format!(
            "https://ptlogin2.qq.com/jump?clientuin={}&clientkey={}&keyindex={}&u1={}",
            self.info.uin, self.key, self.index, url
        )
    }

    pub fn get_mail_url(&self) -> String {
        self.login_url(r"https%3A%2F%2Fmail.qq.com%2Fcgi-bin%2Flogin%3Fvt%3Dpassport%26vm%3Dwpt%26ft%3Dloginpage%26target%3D%26pt_local_tk%3D%26pt_3rd_aid%3D0%26ptopt%3D1%26style%3D25%26org_fun%3D%26aliastype%3D%26ss%3D%26from%3D%26param%3D%26sp%3D%26r%3D65334b69c7dc619a12323e83b566145a%26ppp%3D%26secpp%3D%26tfcont%3D22")
    }

    pub fn get_qzone_url(&self) -> String {
        self.login_url(&format!(
            r"https%3A%2F%2Fuser.qzone.qq.com%2F{}",
            self.get_uin()
        ))
    }

    pub fn get_weiyun_url(&self) -> String {
        self.login_url(r"https%3A%2F%2Fwww.weiyun.com%2Fweb%2Fcallback%2Fcommon_qq_login_ok.html%3Flogin_succ%26pt_local_tk%3D%26pt_3rd_aid%3D0%26ptopt%3D1%26style%3D40")
    }

    pub async fn get_qun_url(&self) -> QQResult<String> {
        let url=self.login_url(r"https%3A%2F%2Fqun.qq.com%2Fmember.html&pt_local_tk=&pt_3rd_aid=0&ptopt=1&style=40&has_onekey=1");
        let response = self
            .client
            .get(url.clone())
            .header(REFERER, QQ_REFERER)
            .send()
            .await?
            .text()
            .await?;
        let url = &Regex::new(r"ptui_qlogin_CB\('0', '(?<url>.*)', ''")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("qun url".into()))?["url"];

        Ok(url.to_string())
    }

    async fn get_skey(&self, url: &String) -> QQResult<(String, String)> {
        let url = url.parse::<Url>()?;
        let query = url.query().ok_or(Error::UrlNoQuery)?;
        let u1 = Regex::new(r"&u1=[^.]*.(?<target>.*)")?;
        let s_url = Regex::new(r"&s_url=.*2F%2F(?<target>[^&]*)")?;
        let mut target = urlencoding::decode(
            &s_url
                .captures(&query)
                .or(u1.captures(&query))
                .ok_or(Error::RegexNoMatch("skey".into()))?["target"],
        )?
        .to_string();
        target.insert_str(0, "https://");
        let target = target.parse::<Url>()?;
        let domain = target.domain().ok_or(Error::UrlNoDomain)?;

        if let Some(v) = self.skey_map.read().await.get(domain) {
            return Ok(v.clone());
        }

        self.client
            .get(url.clone())
            .header(REFERER, QQ_REFERER)
            .send()
            .await?;

        let skey = self
            .cookie_store
            .read()?
            .get("qq.com", "/", "skey")
            .ok_or(Error::CookieNotFound("skey".into()))?
            .value()
            .to_string();
        let p_skey = self
            .cookie_store
            .read()?
            .get(domain, "/", "p_skey")
            .ok_or(Error::CookieNotFound("p_skey".into()))?
            .value()
            .to_string();

        self.skey_map
            .write()
            .await
            .insert(domain.to_string(), (skey.clone(), p_skey.clone()));

        Ok((skey, p_skey))
    }

    pub fn get_nickname(&self) -> Option<String> {
        self.info.nickname.clone()
    }

    pub fn get_uin(&self) -> i64 {
        self.info.uin
    }

    pub async fn get_group_list(&self) -> QQResult<GroupList> {
        let (skey, p_skey) = self.get_skey(&self.get_qun_url().await?).await?;
        let bkn = Self::get_hash(&skey)?;
        let url = "https://qun.qq.com/cgi-bin/qun_mgr/get_group_list";
        let response = self
            .client
            .post(url)
            .form(&[("bkn", bkn.clone())])
            .header(REFERER, QQ_REFERER)
            .send()
            .await?
            .json::<ListResponse>()
            .await?;
        let mut ret = GroupList::new();

        if response.ec != 0 {
            return Err(Error::QQError);
        }

        if let Some(join) = response.join {
            for i in join {
                ret.push(Group::new(
                    self.client.clone(),
                    self.cookie_store.clone(),
                    i,
                    skey.clone(),
                    p_skey.clone(),
                    bkn.clone(),
                    Role::Member,
                ));
            }
        }
        if let Some(manage) = response.manage {
            for i in manage {
                ret.push(Group::new(
                    self.client.clone(),
                    self.cookie_store.clone(),
                    i,
                    skey.clone(),
                    p_skey.clone(),
                    bkn.clone(),
                    Role::Admin,
                ));
            }
        }
        if let Some(create) = response.create {
            for i in create {
                ret.push(Group::new(
                    self.client.clone(),
                    self.cookie_store.clone(),
                    i,
                    skey.clone(),
                    p_skey.clone(),
                    bkn.clone(),
                    Role::Owner,
                ));
            }
        }

        Ok(ret)
    }

    pub async fn get_qzone(&self) -> QQResult<QZone> {
        let (skey, p_skey) = self.get_skey(&self.get_qzone_url()).await?;
        let g_tk = Self::get_hash(&p_skey)?;

        Ok(QZone::new(
            self.client.clone(),
            self.cookie_store.clone(),
            self.get_uin(),
            skey,
            p_skey,
            g_tk,
        ))
    }
}
