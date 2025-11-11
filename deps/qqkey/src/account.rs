use crate::{
    group::{Group, ListResponse, Role},
    qzone::QZone,
    Error, Result as QQResult, QQ,
};
use rand::Rng;
use regex::Regex;
use reqwest::{Method, Url};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct Account {
    qq: Arc<QQ>,
    uin: i64,
    index: i32,
    key: String,
    skey_map: Arc<RwLock<HashMap<String, (String, String)>>>,
}

impl Account {
    pub async fn new(qq: Arc<QQ>, uin: i64) -> QQResult<Self> {
        let (index, key) = Self::get_client_key(qq.clone(), uin).await?;

        Ok(Self {
            qq,
            uin,
            index,
            key,
            skey_map: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn get_client_key(qq: Arc<QQ>, uin: i64) -> QQResult<(i32, String)> {
        let url = format!("https://localhost.ptlogin2.qq.com:4301/pt_get_st?clientuin={}&r={}&pt_local_tk={}&callback=ptui_getst_CB", uin, rand::rng().random_range(0.0..1.0), qq.local_token());
        let response = qq
            .login_request(Method::GET, url)
            .send()
            .await?
            .text()
            .await?;
        let index = Regex::new(r"(?<index>\d*)};")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("client key".into()))?["index"]
            .parse()?;
        let client_key = qq.read_cookie("ptlogin2.qq.com", "/", "clientkey")?;

        Ok((index, client_key))
    }

    fn login_url(&self, url: &str) -> String {
        format!(
            "https://ptlogin2.qq.com/jump?clientuin={}&clientkey={}&keyindex={}&u1={}",
            self.uin, self.key, self.index, url
        )
    }

    pub fn mail_url(&self) -> String {
        self.login_url(r"https%3A%2F%2Fmail.qq.com%2Fcgi-bin%2Flogin%3Fvt%3Dpassport%26vm%3Dwpt%26ft%3Dloginpage%26target%3D%26pt_local_tk%3D%26pt_3rd_aid%3D0%26ptopt%3D1%26style%3D25%26org_fun%3D%26aliastype%3D%26ss%3D%26from%3D%26param%3D%26sp%3D%26r%3D65334b69c7dc619a12323e83b566145a%26ppp%3D%26secpp%3D%26tfcont%3D22")
    }

    pub fn qzone_url(&self) -> String {
        self.login_url(&format!(
            r"https%3A%2F%2Fuser.qzone.qq.com%2F{}",
            self.uin()
        ))
    }

    pub fn weiyun_url(&self) -> String {
        self.login_url(r"https%3A%2F%2Fwww.weiyun.com%2Fweb%2Fcallback%2Fcommon_qq_login_ok.html%3Flogin_succ%26pt_local_tk%3D%26pt_3rd_aid%3D0%26ptopt%3D1%26style%3D40")
    }

    fn hash_skey(skey: &String) -> QQResult<String> {
        let mut t = 5381_i32;
        let o = skey.len();

        for n in 0..o {
            (t, _) = t.overflowing_add(
                (t << 5) + skey.chars().nth(n).ok_or(Error::InvalidCharacter)? as i32,
            );
        }

        Ok((t & 0x7fffffff).to_string())
    }

    pub async fn qun_url(&self) -> QQResult<String> {
        let url=self.login_url(r"https%3A%2F%2Fqun.qq.com%2Fmember.html&pt_local_tk=&pt_3rd_aid=0&ptopt=1&style=40&has_onekey=1");
        let response = self
            .qq
            .qq_request(Method::GET, url)
            .send()
            .await?
            .text()
            .await?;

        Ok(Regex::new(r"ptui_qlogin_CB\('0', '(?<url>.*)', ''")?
            .captures(&response)
            .ok_or(Error::RegexNoMatch("qun url".into()))?["url"]
            .to_string())
    }

    async fn skey(&self, url: &String) -> QQResult<(String, String)> {
        let url = url.parse::<Url>()?;
        let query = url.query().ok_or(Error::UrlNoQuery)?;
        let u1 = Regex::new(r"&u1=[^.]*.(?<target>.*)")?;
        let s_url = Regex::new(r"&s_url=.*2F%2F(?<target>[^&]*)")?;
        let mut target = urlencoding::decode(
            &s_url
                .captures(query)
                .or(u1.captures(query))
                .ok_or(Error::RegexNoMatch("skey".into()))?["target"],
        )?
        .to_string();
        target.insert_str(0, "https://");
        let target = target.parse::<Url>()?;
        let domain = target.domain().ok_or(Error::UrlNoDomain)?;

        if let Some(v) = self.skey_map.read().await.get(domain) {
            return Ok(v.clone());
        }

        self.qq.qq_request(Method::GET, url).send().await?;

        let skey = self.qq.read_cookie("qq.com", "/", "skey")?;
        let p_skey = self.qq.read_cookie(domain, "/", "p_skey")?;

        self.skey_map
            .write()
            .await
            .insert(domain.to_string(), (skey.clone(), p_skey.clone()));

        Ok((skey, p_skey))
    }

    pub fn uin(&self) -> i64 {
        self.uin
    }

    pub async fn group_list(&self) -> QQResult<Vec<Group>> {
        let (skey, _) = self.skey(&self.qun_url().await?).await?;
        let bkn = Self::hash_skey(&skey)?;
        let url = "https://qun.qq.com/cgi-bin/qun_mgr/get_group_list";
        let response = self
            .qq
            .qq_request(Method::POST, url)
            .form(&[("bkn", bkn.clone())])
            .send()
            .await?
            .json::<ListResponse>()
            .await?;
        let mut ret = Vec::new();

        if response.ec != 0 {
            return Err(Error::RequestQQError);
        }

        if let Some(join) = response.join {
            ret.extend(
                join.into_iter()
                    .map(|i| Group::new(self.qq.clone(), i, bkn.clone(), Role::Member)),
            );
        }
        if let Some(manage) = response.manage {
            ret.extend(
                manage
                    .into_iter()
                    .map(|i| Group::new(self.qq.clone(), i, bkn.clone(), Role::Admin)),
            );
        }
        if let Some(create) = response.create {
            ret.extend(
                create
                    .into_iter()
                    .map(|i| Group::new(self.qq.clone(), i, bkn.clone(), Role::Owner)),
            );
        }

        Ok(ret)
    }

    pub async fn qzone(&self) -> QQResult<QZone> {
        let (_, p_skey) = self.skey(&self.qzone_url()).await?;
        let g_tk = Self::hash_skey(&p_skey)?;

        Ok(QZone::new(self.qq.clone(), self.uin, g_tk))
    }
}
